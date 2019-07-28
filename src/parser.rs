use combine::{Parser, none_of, many, skip_many, token, between, choice, parser};
use combine::parser::char::char;
use combine::error::{ParseError};
use combine::stream::{Stream};

#[derive(PartialEq, Debug, Clone)]
pub enum Instr {
    MLeft,
    MRight,
    Incr,
    Decr,
    Stdout,
    Stdin,
    Loop(Vec<Instr>)
}

fn comments<I>() -> impl Parser<Input = I, Output = ()>
where
   I: Stream<Item = char>,
   I::Error: ParseError<I::Item, I::Range, I::Position>
{
    skip_many(none_of("+-<>[].,".chars()))
}

fn parse_loop<I>() -> impl Parser<Input = I, Output = Instr>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>
{
    between(token('['), token(']'), many(instr()))
        .map(Instr::Loop)
        .skip(comments())
}

fn instr_<I>() -> impl Parser<Input = I, Output = Instr>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>
{

    let incr = char('+').map(|_| Instr::Incr);
    let decr = char('-').map(|_| Instr::Decr);
    let left = char('<').map(|_| Instr::MLeft);
    let right = char('>').map(|_| Instr::MRight);
    let stdin = char(',').map(|_| Instr::Stdin);
    let stdout = char('.').map(|_| Instr::Stdout);

    choice((
        parse_loop(),
        incr,
        decr,
        left,
        right,
        stdin,
        stdout
    ))
}

parser!{
    pub fn instr[I]()(I) -> Instr
    where [I: Stream<Item = char>]
    {
        instr_()
            .skip(comments())
    }
}

pub fn instrs<I>() -> impl Parser<Input = I, Output = Vec<Instr>>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>
{
    many(instr())
}

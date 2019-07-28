#[macro_use] extern crate combine;


use crate::combine::Parser;
mod parser;
mod logger;
use logger::prompt;
mod vm;
use vm::Vm;

fn run_code(input: &str) {
    let mut vm = Vm::new(3000);
    let result = parser::instrs().parse(input);
    match result {
        Err(_) => logger::error("Cannot parse brainfuck code"),
        Ok((ast, _)) => {
            if let Err(e) = vm.run(ast) {
                logger::error(format!("Error: {:?}", e));
            }
        }
    };
}

fn main() {
    let (arguments, _) = cmdparser::Parser::new().parse();
    if let Some(input) = arguments.get("c") {
        return run_code(&input[0]);
    }
    loop {
        match prompt("Enter brainfuck code") {
            Err(e) => logger::error(format!("Error: {:?}", e)),
            Ok(input) => run_code(&input)
        };
    }
}

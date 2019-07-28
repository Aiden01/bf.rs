#[macro_use]
extern crate combine;

use crate::combine::Parser;
mod parser;
mod logger;
use logger::prompt;

fn main() {
    loop {
        let input = prompt("Enter brainfuck code");
        let result = parser::instrs().parse(input.as_str());
        dbg!(result);
    }
}

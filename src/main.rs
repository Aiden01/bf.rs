#[macro_use]
extern crate combine;

use crate::combine::Parser;
mod parser;
use parser::instrs;
mod logger;
use logger::prompt;
mod vm;
use vm::Vm;

fn main() {
    let mut vm = Vm::new(3000);
    loop {
        let input = prompt("Enter brainfuck code");
        let result = parser::instrs().parse(input.as_str());
        match result {
            Err(e) => logger::error("Cannot parse brainfuck code"),
            Ok((ast, _)) => {
                if let Err(e) = vm.run(ast) {
                    logger::error(format!("Error: {:?}", e));
                }
            }
        };
    }
}

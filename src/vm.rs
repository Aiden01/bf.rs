use crate::logger::{error, prompt};
use crate::parser::Instr;
use std::cmp::{max, min};
use std::io::{stdout, Write};
use Instr::*;

pub struct Vm {
    max_memory: usize,
    memory: Vec<usize>,
    pointer: usize,
}

type VmResult<T> = Result<T, String>;

impl Vm {
    pub fn new(max_memory: usize) -> Self {
        Vm {
            max_memory,
            memory: vec![0; max_memory],
            pointer: 0,
        }
    }

    pub fn run(&mut self, instrs: Vec<Instr>) -> VmResult<()> {
        for instr in instrs {
            let result = self.execute_instr(instr)?;
        }

        Ok(())
    }

    fn get_cell(&mut self) -> VmResult<&mut usize> {
        self.memory
            .get_mut(self.pointer)
            .ok_or(format!("Pointer is out of bound: {}", self.pointer))
    }

    fn set_cell(&mut self, x: usize) -> VmResult<()> {
        let cell = self.get_cell()?;
        *cell = x;
        Ok(())
    }

    fn move_ptr(&mut self, direction: Instr) -> VmResult<()> {
        self.pointer = match direction {
            MLeft => max(0, self.pointer - 1),
            MRight => min(self.max_memory, self.pointer + 1),
            _ => unreachable!(),
        };

        Ok(())
    }

    fn incr(&mut self) -> VmResult<()> {
        let cell = self.get_cell()?;
        *cell += 1;
        Ok(())
    }

    fn decr(&mut self) -> VmResult<()> {
        let cell = self.get_cell()?;
        *cell -= 1;
        Ok(())
    }

    fn execute_loop(&mut self, instrs: Vec<Instr>) -> VmResult<()> {
        let mut cell = self.get_cell()?;
        while *cell != 0 {
            self.run(instrs.clone())?;
            cell = self.get_cell()?;
        }

        Ok(())
    }

    fn stdout(&mut self) -> VmResult<()> {
        let cell = self.get_cell()?;
        print!("{}", *cell as u8 as char);
        Ok(())
    }

    fn stdin(&mut self) -> VmResult<()> {
        let mut line = String::new();
        line = prompt("Enter char");
        while line.len() != 2 {
            error("Please enter a single char.");
            line = prompt("Enter char");
        }

        let c = line.as_bytes()[0];
        self.set_cell(c as usize)
    }

    fn execute_instr(&mut self, instr: Instr) -> VmResult<()> {
        match instr {
            d @ MLeft | d @ MRight => self.move_ptr(d),
            Incr => self.incr(),
            Decr => self.decr(),
            Loop(instrs) => self.execute_loop(instrs),
            Stdout => self.stdout(),
            Stdin => self.stdin(),
        }
    }
}

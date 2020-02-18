use self::Instruction::*;
use std::cmp::max;
use std::iter::Iterator;
use std::vec::Vec;
use IntoIterator;

#[derive(Debug)]
pub enum Instruction {
    Zero(usize),
    Increase(usize),
    Translate {
        from: usize,
        to: usize,
    },
    Jump {
        first: usize,
        second: usize,
        goto: usize,
    },
}

#[derive(Debug)]
pub struct Environment {
    app: Vec<Instruction>,
    registers: Vec<u64>,
    line: usize,
}

impl Environment {
    pub fn new(instructions: impl IntoIterator<Item = Instruction>) -> Self {
        let instructions: Vec<Instruction> = instructions.into_iter().collect();
        let regs = initialize_registers(&instructions);
        Environment {
            app: instructions,
            registers: regs,
            line: 1,
        }
    }

    pub fn regs(&self) -> Vec<u64> {
        self.registers.clone()
    }
}

fn initialize_registers(instructions: &Vec<Instruction>) -> Vec<u64> {
    let max_register = instructions
        .iter()
        .map(|x| match *x {
            Zero(r) | Increase(r) => r,
            Translate { from, to } => max(from, to),
            Jump { first, second, .. } => max(first, second),
        })
        .max()
        .unwrap_or(0);
    vec![0; max_register + 1]
}

impl Iterator for Environment {
    type Item = (usize, Vec<u64>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.line == 0 {
            None
        } else {
            let instruction = self.app.get(self.line - 1)?;
            self.line = match *instruction {
                Increase(r) => {
                    self.registers[r] += 1;
                    self.line + 1
                }
                Zero(r) => {
                    self.registers[r] = 0;
                    self.line + 1
                }
                Translate { from, to } => {
                    self.registers[to] = self.registers[from];
                    self.line + 1
                }
                Jump {
                    first,
                    second,
                    goto,
                } => {
                    if self.registers[first] == self.registers[second] {
                        goto
                    } else {
                        self.line + 1
                    }
                }
            };
            Some((self.line, self.regs()))
        }
    }
}

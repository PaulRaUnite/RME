use self::Instruction::*;
use std::cmp::max;
use std::iter::Iterator;
use std::vec::Vec;

#[derive(Debug)]
pub enum Instruction {
    Increase(usize),
    Zero(usize),
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
pub struct Environment<'a> {
    app: &'a Vec<Instruction>,
    registers: Vec<u64>,
    line: usize,
}

impl<'a> Environment<'a> {
    pub fn new(instr: &'a Vec<Instruction>) -> Self {
        let regs = initialize_registers(instr);
        Environment {
            app: instr,
            registers: regs,
            line: 0,
        }
    }

    pub fn regs(&'a self) -> Vec<u64> {
        self.registers.clone()
    }
}

fn initialize_registers(instructions: &Vec<Instruction>) -> Vec<u64> {
    let mut max_register: usize = 0;
    for instruction in instructions {
        match *instruction {
            Increase(r) => {
                max_register = max(max_register, r);
            }
            Zero(r) => {
                max_register = max(max_register, r);
            }
            Translate { from, to } => {
                max_register = max(max_register, from);
                max_register = max(max_register, to);
            }
            Jump { first, second, .. } => {
                max_register = max(max_register, first);
                max_register = max(max_register, second);
            }
        }
    }
    vec![0; max_register + 1]
}

impl<'a> Iterator for Environment<'a> {
    type Item = (usize, &'a Vec<Instruction>, Vec<u64>);

    fn next<'b>(&'b mut self) -> Option<(usize, &'a Vec<Instruction>, Vec<u64>)> {
        if self.line < self.app.len() {
            let prev = self.line;
            self.line = match self.app[self.line] {
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
            Some((prev, self.app, self.regs()))
        } else {
            None
        }
    }
}

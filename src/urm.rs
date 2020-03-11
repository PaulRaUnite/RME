use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug)]
pub enum Instruction {
    Zero(usize),
    Increase(usize),
    Translate {
        from: usize,
        to: usize,
    },
    Jump {
        reg1: usize,
        reg2: usize,
        goto: usize,
    },
}

pub enum Memory {
    Linear(Vec<u64>),
    Sparse(HashMap<usize, u64>),
}

pub struct Program(Vec<Instruction>);

impl Memory {
    pub fn new_linear(size: usize) -> Self {
        let mut vec = Vec::with_capacity(size);
        vec.resize(size, 0);
        Memory::Linear(vec)
    }

    pub fn new_sparse<'a>(active_registers: impl IntoIterator<Item = &'a usize>) -> Self {
        Memory::Sparse(HashMap::from_iter(
            active_registers.into_iter().map(|x| (*x, 0)),
        ))
    }

    pub fn new<'a>(active_registers: impl IntoIterator<Item = &'a usize>) -> Self {
        Self::new_sparse(active_registers)
    }
}

use itertools::{enumerate, Itertools};
use pest::Parser;
use std::fmt;
use std::fmt::{Debug, Error, Formatter};

#[derive(Parser)]
#[grammar = "urm.pest"]
struct URMParser;

pub fn parse(program: &str) -> Result<Program, Box<dyn std::error::Error>> {
    let pairs = URMParser::parse(Rule::program, program)?
        .next()
        .ok_or("no input")?
        .into_inner();

    let mut instructions = Vec::with_capacity(program.lines().count());
    for pair in pairs {
        instructions.push(match pair.as_rule() {
            Rule::jump => {
                let (first, second, goto) = pair
                    .into_inner()
                    .map(|x| x.as_str().parse())
                    .next_tuple()
                    .unwrap();
                Instruction::Jump {
                    reg1: first?,
                    reg2: second?,
                    goto: goto?,
                }
            }
            Rule::succ => {
                let register = pair
                    .into_inner()
                    .map(|x| x.as_str().parse())
                    .next()
                    .unwrap()?;
                Instruction::Increase(register)
            }
            Rule::zero => {
                let register = pair
                    .into_inner()
                    .map(|x| x.as_str().parse())
                    .next()
                    .unwrap()?;
                Instruction::Zero(register)
            }
            Rule::tran => {
                let (reg1, reg2) = pair
                    .into_inner()
                    .map(|x| x.as_str().parse())
                    .next_tuple()
                    .unwrap();
                Instruction::Translate {
                    from: reg1?,
                    to: reg2?,
                }
            }
            Rule::num => panic!("should not be in the context"),
            Rule::EOI | Rule::program | Rule::WHITESPACE => continue,
        });
    }
    Ok(Program(instructions))
}

impl fmt::Debug for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let last_elem = self.0.len() - 1;
        for (index, instr) in enumerate(self.0.iter()) {
            write!(f, "{:?}", instr)?;
            if index != last_elem {
                writeln!(f)?
            }
        }
        Ok(())
    }
}

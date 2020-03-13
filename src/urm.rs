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

#[derive(Debug)]
pub enum Memory {
    Linear(Vec<u64>),
    Sparse(HashMap<usize, u64>),
}

pub struct Program(Vec<Instruction>);

impl Memory {
    pub fn new_linear(size: usize) -> Self {
        Memory::Linear(vec![0; size])
    }

    pub fn new_sparse<'a>(active_registers: impl Iterator<Item = &'a usize>) -> Self {
        Memory::Sparse(HashMap::from_iter(active_registers.map(|x| (*x, 0))))
    }

    pub fn from_program(program: &Program) -> Self {
        Self::new(program.iter_registers())
    }

    pub fn new<'a>(active_registers: impl Iterator<Item = &'a usize>) -> Self {
        Self::new_sparse(active_registers)
    }
}

#[derive(Debug)]
pub struct Application {
    memory: Memory,
    program: Program,
}

impl Application {
    pub fn from_str(content: &str) -> Result<Application, Box<dyn std::error::Error>> {
        let program = parse(content)?;
        let memory = Memory::from_program(&program);
        Ok(Application { memory, program })
    }

    pub fn run(&mut self, arguments: &[u64]) -> Result<u64, Box<dyn std::error::Error>> {
        Ok(0)
    }
}

impl Index<usize> for Memory {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Memory::Linear(vector) => vector.index(index),
            Memory::Sparse(map) => map.index(&index),
        }
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self {
            Memory::Linear(vector) => vector.index_mut(index),
            Memory::Sparse(map) => map.get_mut(&index).unwrap(),
        }
    }
}

use itertools::{enumerate, Itertools};
use pest::Parser;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut};

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
                    .map(|x| x.as_str().parse().unwrap())
                    .next_tuple()
                    .unwrap();
                Instruction::Jump {
                    reg1: first,
                    reg2: second,
                    goto,
                }
            }
            Rule::succ => {
                let register = pair
                    .into_inner()
                    .map(|x| x.as_str().parse().unwrap())
                    .next()
                    .unwrap();
                Instruction::Increase(register)
            }
            Rule::zero => {
                let register = pair
                    .into_inner()
                    .map(|x| x.as_str().parse().unwrap())
                    .next()
                    .unwrap();
                Instruction::Zero(register)
            }
            Rule::tran => {
                let (reg1, reg2) = pair
                    .into_inner()
                    .map(|x| x.as_str().parse().unwrap())
                    .next_tuple()
                    .unwrap();
                Instruction::Translate {
                    from: reg1,
                    to: reg2,
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

impl Program {
    pub fn iter_registers(&self) -> impl Iterator<Item = &usize> {
        self.0.iter().flat_map(|i| i.iter_registers())
    }
}

impl Instruction {
    pub fn iter_registers(&self) -> impl Iterator<Item = &usize> {
        InstructionRegisters {
            instruction: self,
            index: 0,
        }
    }
}

struct InstructionRegisters<'a> {
    instruction: &'a Instruction,
    index: u8,
}

impl<'a> Iterator for InstructionRegisters<'a> {
    type Item = &'a usize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.instruction {
            Instruction::Increase(register) | Instruction::Zero(register) => match self.index {
                0 => Some(register),
                _ => None,
            },
            Instruction::Translate {
                from: reg1,
                to: reg2,
            }
            | Instruction::Jump { reg1, reg2, .. } => match self.index {
                0 => Some(reg1),
                1 => Some(reg2),
                _ => None,
            },
        };
        if result.is_some() {
            self.index += 1
        }
        result
    }
}

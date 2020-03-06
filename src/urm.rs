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
        first: usize,
        second: usize,
        goto: usize,
    },
}

pub enum Memory {
    Linear(Vec<u64>),
    Sparse(HashMap<usize, u64>),
}

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

use pest::Parser;
use pest::error::Error;

#[derive(Parser)]
#[grammar = "urm.pest"]
struct URMParser;

pub fn parse(program: &str) -> Result<Vec<Instruction>, Error<Rule>> {
    let pairs = URMParser::parse(Rule::program, program)?;

    let mut instructions = Vec::with_capacity(program.lines().count());
    for pair in pairs {
        instructions.push(match pair { })
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());
    }
}

extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::path::PathBuf;
use structopt::StructOpt;

use std::error::Error;
use std::fs::read_to_string;

mod urm;

/// Register Machine Environment
#[derive(StructOpt, Debug)]
#[structopt(name = "rme")]
struct Opt {
    /// Path to the Register Machine program
    program: PathBuf,
    /// Arguments of the program
    values: Vec<u64>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let program_code = read_to_string(opt.program)?;
    let mut interpretation = urm::Application::from_str(&program_code)?;
    println!("{:?}", interpretation.run(&opt.values)?);
    Ok(())
}

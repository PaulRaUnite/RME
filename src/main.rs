extern crate clap;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::error::Error;
use std::fs::File;
use std::io::Read;

use clap::Arg;

mod urm;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap::App::new("URM interpreter")
        .version("0.1.0")
        .author("Pavlo Tokariev")
        .arg(
            Arg::with_name("FILE")
                .index(1)
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("values")
                .index(2)
                .required(false)
                .takes_value(true)
                .multiple(true),
        )
        .get_matches();

    let filename = matches.value_of("FILE").unwrap();
    let values = matches
        .values_of("values")
        .unwrap()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();

    let content = read_file(filename)?;

    println!("{:?}", urm::Application::from_str(&content)?.run(&values)?);
    Ok(())
}

fn read_file(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate clap;

use clap::Arg;
use std::error::Error;
use std::fs::File;
use std::io::Read;

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
        .get_matches();

    let filename = matches.value_of("FILE").unwrap();

    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let pairs = URMParser::parse(Rule::program, &content)?;
    for pair in pairs {
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());
    }
    Ok(())
}

use colored::*;
use parser::Parser;
use std::{error::Error, io, io::prelude::*};

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match Parser::new(&input.trim_end()).parse() {
            Ok(output) => {
                for node in output {
                    println!("{:#?}", node)
                }
            }
            Err(error) => println!("{}: {}", "Error while parsing".red(), error),
        }
    }
}

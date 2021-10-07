use parser::Parser;
use std::{error::Error, io, io::prelude::*};

enum Status {
    Incomplete,
    New,
}

impl Status {
    fn prompt(&self) -> &'static str {
        match self {
            Status::Incomplete => "... ",
            Status::New => ">>> ",
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut status = Status::New;

    loop {
        io::stdout().write_all(status.prompt().as_bytes())?;
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        // If we receive 2 empty lines, we should stop accepting input and parse.
        status = match input.lines().last().map_or(false, str::is_empty) {
            true => Status::New,
            false => Status::Incomplete,
        };

        if let Status::New = status {
            parse_and_run(&input);
            input.clear();
            input.shrink_to_fit();
        }
    }
}

fn parse_and_run(input: &str) {
    let mut parser = Parser::new(input);
    let result = parser.parse();
    println!("{:#?}", result);

    for error in parser.errors() {
        println!("{}", error.details());
    }
}

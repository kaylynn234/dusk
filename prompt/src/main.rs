use parser::Parser;
use std::{error::Error, io, io::prelude::*, sync::Arc};

const PROMPT_NEW: &[u8] = b">>> ";
const PROMPT_INCOMPLETE: &[u8] = b"... ";

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let mut current_prompt = PROMPT_NEW;

    loop {
        io::stdout().write_all(current_prompt)?;
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        // If we receive 2 empty lines, we should stop accepting input and parse.
        current_prompt = {
            if input.lines().next_back().map_or(false, str::is_empty) {
                parse_and_run(&input);
                input.clear();
                PROMPT_NEW
            } else {
                PROMPT_INCOMPLETE
            }
        };
    }
}

fn parse_and_run(input: &str) {
    // This is gross.
    match Parser::new(&Arc::new(input.trim_end().to_owned()), "<stdin>".to_owned()).parse() {
        Ok(output) => {
            for node in output {
                println!("{:#?}", node)
            }
        }
        Err(error) => println!("{}", error),
    }
}

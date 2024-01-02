use anyhow::Result;
use clap::Parser;
use std::io::{stdin, stdout, Write};

use crate::lexer::FpsInput;

mod lexer;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// To access the REPL
    #[arg(short, long)]
    repl: bool,
}

fn execute(input: &str) -> Result<()> {
    let mut scanner = FpsInput::new(input);
    scanner.scan_tokens()?;

    for token in scanner.tokens {
        println!("{:?}", token);
    }

    Ok(())
}

fn run_prompt() -> Result<()> {
    println!("REPL for FPS Lang");
    println!("-----------------");
    println!("Type '\\q' or press 'Ctrl+Z' to exit");
    let mut quit = false;
    let mut buffer = String::new();
    while !quit {
        buffer.clear();
        print!("fps> ");
        stdout().flush()?;
        stdin().read_line(&mut buffer)?;
        // remove LF
        // buffer = buffer.as_str().trim_end().to_string();
        execute(&buffer)?;
        if buffer == "\\q" {
            quit = true;
        }

        execute(&buffer)?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();

    if !args.repl {
        let _ = include_str!("sample.fps");
        todo!()
    } else {
        run_prompt()?
    }

    Ok(())
}

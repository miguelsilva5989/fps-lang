use anyhow::Result;
use clap::Parser as ClapParser;
use parser::Parser;
use std::io::{stdin, stdout, Write};

use crate::lexer::FpsInput;

mod lexer;
mod ast;
mod parser;

#[derive(ClapParser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// To access the REPL
    #[arg(short, long)]
    repl: bool,
}

fn execute(input: &str) -> Result<()> {
    let mut scanner = FpsInput::new(input);
    scanner.scan_tokens()?;

    let mut parser = Parser::new(scanner.tokens);
    let expression = parser.expression()?;

    println!("{}", expression);

    // for token in scanner.tokens {
    //     println!("Token {}", token);
    // }

    Ok(())
}

fn run_prompt() -> Result<()> {
    println!("REPL for FPS Lang");
    println!("-----------------");
    println!("Type '\\q' or press 'Ctrl+Z' to exit");
    let mut buffer = String::new();
    loop {
        buffer.clear();
        print!("fps> ");
        stdout().flush()?;
        stdin().read_line(&mut buffer)?;
        // remove LF
        buffer = buffer.as_str().trim_end().to_string();

        if buffer == "\\q" {
            break;
        }

        execute(&buffer)?;
    }
    Ok(())
}

// fn run_file(input: &str) -> Result<()> {
//     todo!("fix line POS");
//     let mut scanner = FpsInput::new(input);
//     for line in input.lines() {
//         scanner.scan_tokens()?;
//     }

//     for token in scanner.tokens {
//         println!("Token {}", token);
//     }

//     Ok(())
// }

fn main() -> Result<()> {
    let args = Cli::parse();

    if !args.repl {
        todo!();
        // let input = include_str!("sample.fps");
        // run_file(input)?;
    } else {
        run_prompt()?
    }

    Ok(())
}

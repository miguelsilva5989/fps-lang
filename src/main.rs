use std::str;
use anyhow::Result;
use clap::Parser as ClapParser;
use std::io::{stdin, stdout, Write};

use crate::interpreter::Interpreter;
use crate::lexer::FpsInput;
use crate::parser::Parser;

mod ast;
mod interpreter;
mod lexer;
mod parser;

#[derive(ClapParser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// flag to access the REPL
    #[arg(short, long)]
    repl: bool,
}

fn execute(interpreter: &mut Interpreter, input: &str) -> Result<()> {
    let mut stdout = Vec::new();
    
    let mut scanner = FpsInput::new(input);
    scanner.scan_tokens()?;

    // println!("{}", scanner);

    let mut parser = Parser::new(scanner.tokens);

    let res = parser.parse();
    match res {
        Ok(statements) => {
            // println!("statements: {:?}", statements);
            interpreter.interpret(&mut stdout, statements)?;
        }
        Err(res) => println!("ERROR: {:?}", res),
    }

    match str::from_utf8(&stdout) {
        Ok(val) => println!("{}", val),
        Err(err) => panic!("{}", err.to_string())
    }
    

    

    // for statement in statements {

    //     // match statement {
    //     //     ast::statement::Statement::ArithmeticExpr(expr) => {
    //     //         interpreter.interpret_expr(expr)?;
    //     //     }
    //     //     // ast::statement::Statement::Print(_) => todo!(),
    //     //     // ast::statement::Statement::Assign { id, expr } => todo!(),
    //     //     _ => todo!(),
    //     // }

    //     // let result =
    // }

    Ok(())
}

fn run_prompt() -> Result<()> {
    println!("# REPL  -  FPS Lang #");
    println!("--------------------");
    println!("Type '\\q' to exit");
    let mut buffer = String::new();
    let mut interpreter: Interpreter = Interpreter::new();
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

        execute(&mut interpreter, &buffer)?;
    }
    Ok(())
}

fn run_file(input: &str) -> Result<()> {
    let mut interpreter: Interpreter = Interpreter::new();
    execute(&mut interpreter, input)?;

    // todo!("fix line POS and multiline processing");

    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();

    if !args.repl {
        // todo!();
        let input = include_str!("sample.fps");
        run_file(input)?;
    } else {
        run_prompt()?
    }

    Ok(())
}

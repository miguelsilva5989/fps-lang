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
    /// To access the REPL
    #[arg(short, long)]
    repl: bool,
}

fn execute(interpreter: &mut Interpreter, input: &str, is_repl: bool) -> Result<()> {
    let mut scanner = FpsInput::new(input);
    scanner.scan_tokens()?;

    println!("{}", scanner);

    let mut parser = Parser::new(scanner.tokens);

    let res = parser.parse(is_repl);
    match res {
        Ok(statements) => {
            // println!("statements: {:?}", statements);
            interpreter.interpret(statements)?;
        }
        Err(res) => println!("ERROR: {:?}", res),
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
    println!("----------_---------");
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

        execute(&mut interpreter, &buffer, true)?;
    }
    Ok(())
}

fn run_file(input: &str) -> Result<()> {
    let mut interpreter: Interpreter = Interpreter::new();
    execute(&mut interpreter, input, false)?;

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

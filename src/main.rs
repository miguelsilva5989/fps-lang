use clap::Parser;
use std::io::{stdin, stdout, Write};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// To access the REPL
    #[arg(short, long)]
    repl: bool,
}

fn main() {
    let args = Cli::parse();

    if !args.repl {
        let _ = include_str!("sample.fps");
    } else {
        println!("REPL for FPS Lang");
        println!("-----------------");
        println!("Type '\\q' or press 'Ctrl+Z' to exit");
        let mut quit = false;
        let mut statement = String::new();
        while !quit {
            statement.clear();
            print!("fps> ");
            stdout().flush().unwrap();
            if let 0 = stdin().read_line(&mut statement).unwrap() {
                return;
            };
            statement = statement.as_str().trim_end().to_string();
            if statement == "\\q" {
                quit = true;
            }
        }
    }
}

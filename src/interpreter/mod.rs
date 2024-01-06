use std::rc::Rc;
use std::io;

use crate::ast::fps::Fps;
use crate::ast::{environment::Environment, statement::Statement};

use anyhow::Result;

#[derive(Debug)]
pub struct Interpreter {
    environment: Environment,
    fps: Fps,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
            fps: Fps::new(),
        }
    }

    pub fn interpret(&mut self, stdout: &mut dyn io::Write, statements: Vec<Statement>) -> Result<()> {
        

        // println!("statements: {:?}", statements);
        self.fps.allocate_statements_to_frame(statements)?;

        // for (range, statements) in self.frames.clone() {
        //     println!("{:?}", range);
        //     for statement in statements {
        //         println!("\t{:?}", statement);
        //     }
        // }

        println!("{:?}", self.fps.frames.get(&1));

        for (frame, range_statements) in self.fps.frames.clone() {
            for statement in range_statements {
                match statement {
                    Statement::Fps(_) => {}
                    Statement::FpsEnd(_) => {}
                    Statement::Comment(_) => {}
                    Statement::ArithmeticExpr(expr) => {
                        expr.eval(&mut self.environment)?;
                    }
                    Statement::Print(expr) => {
                        let value = expr.eval(&mut self.environment)?;
                        writeln!(stdout, "FPS {} -> {}", frame, value).unwrap();
                    }
                    Statement::Declaration { id, expr } => {
                        let value = expr.eval(&mut self.environment)?;

                        self.environment.declare(id.lexeme, value)?;
                        // println!("{value}");
                    }
                    Statement::Block { statements } => {
                        let mut new_env = Environment::new();
                        new_env.parent = Some(Rc::from(self.environment.clone()));
                        let old_env = self.environment.clone();
                        self.environment = new_env;
                        self.interpret(stdout, statements)?;
                        self.environment = old_env;
                    }
                };
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::FpsInput, parser::Parser};

    use super::*;
    #[test]
    fn multiple_frames() {
        let mut stdout = Vec::new();

        let input = "let a = 1; #2 a = a + 1; # print(a); ##";
        let expected = "FPS 4 -> 3\n".as_bytes();

        let mut scanner = FpsInput::new(input);
        scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(scanner.tokens);
        let statements = parser.parse().unwrap();
        
        let mut interpreter: Interpreter = Interpreter::new();
        interpreter.interpret(&mut stdout, statements).unwrap();

        assert_eq!(stdout, expected)
    }
}

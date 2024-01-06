use std::rc::Rc;

use crate::ast::fps::Fps;
use crate::ast::{environment::Environment, statement::Statement};

use anyhow::Result;

#[derive(Debug)]
pub struct Interpreter {
    environment: Environment,
    fps: Fps,
    current_fps: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
            fps: Fps::new(),
            current_fps: 0,
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<()> {
        // println!("statements: {:?}", statements);
        self.fps.allocate_statements_to_frame(statements)?;

        // for (range, statements) in self.frames.clone() {
        //     println!("{:?}", range);
        //     for statement in statements {
        //         println!("\t{:?}", statement);
        //     }
        // }

        for (_, range_statements) in self.fps.frames.clone() {
            for statement in range_statements {
                // println!("{:?}", statement);

                match statement {
                    Statement::Fps(_) => {}
                    Statement::FpsEnd(_) => {}
                    Statement::Comment(_) => {}
                    Statement::ArithmeticExpr(expr) => {
                        expr.eval(&mut self.environment)?;
                    }
                    Statement::Print(expr) => {
                        let value = expr.eval(&mut self.environment)?;
                        println!("{value}");
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
                        self.interpret(statements)?;
                        self.environment = old_env;
                    }
                };
            }
        }

        Ok(())
    }
}

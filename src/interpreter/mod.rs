use std::rc::Rc;

use crate::ast::{environment::Environment, statement::Statement};

use anyhow::Result;

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<()> {
        for statement in statements {
            match statement {
                Statement::Fps(fps) => {
                    println!("FPS {}", fps)
                }
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

        Ok(())
    }
}

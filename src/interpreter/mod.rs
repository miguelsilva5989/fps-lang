use crate::ast::{env::Environment, statement::Statement};

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
                Statement::ArithmeticExpr(expr) => {
                    expr.eval(&self.environment)?;
                }
                Statement::Print(expr) => {
                    let value = expr.eval(&self.environment)?;
                    println!("{value}");
                }
                Statement::Declaration { id, expr } => {
                    let value = expr.eval(&self.environment)?;

                    self.environment.declare(id.lexeme, value)?;
                    // println!("{value}");
                }
            };
        }

        Ok(())
    }
}

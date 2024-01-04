use crate::ast::{arithmetic::Expr, literal::LiteralValue, statement::Statement};

use anyhow::Result;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret_expr(&mut self, expr: Expr) -> Result<LiteralValue> {
        expr.evaluate()
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<()> {
        for statement in statements {
            match statement {
                Statement::ArithmeticExpr(expr) => {
                    expr.evaluate()?;
                }
                Statement::Print(expr) => {
                    let value = expr.evaluate()?;
                    println!("{value}");
                }
                Statement::Assign { id, expr } => todo!(),
            };
        }

        Ok(())
    }
}

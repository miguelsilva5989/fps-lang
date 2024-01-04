use crate::ast::{arithmetic::Expr, literal::LiteralValue};

use anyhow::Result;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self { Self {  } }

    pub fn interpret(&mut self, expr: Expr) -> Result<LiteralValue> {
        expr.evaluate()
    }
}
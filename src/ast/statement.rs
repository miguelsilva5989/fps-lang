use crate::lexer::Token;
use super::expr::Expr;


#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Fps(Token),
    FpsEnd(Token),
    Comment(Token),
    ArithmeticExpr(Expr),
    Print(Expr),
    Declaration { id: Token, expr: Expr },
    Block {statements: Vec<Statement>, }
}

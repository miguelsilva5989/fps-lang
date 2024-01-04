use crate::lexer::Token;
use super::expr::Expr;


#[derive(Debug, PartialEq)]
pub enum Statement {
    ArithmeticExpr(Expr),
    Print(Expr),
    Declaration { id: Token, expr: Expr },
    Fps(Token)
}

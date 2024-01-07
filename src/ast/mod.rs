use thiserror::Error;

pub mod expr;
pub mod literal;
pub mod statement;
pub mod environment;
pub mod fps;

use crate::lexer::{self, Token, TokenType};
use literal::LiteralValue;

#[derive(Error, Debug)]
enum AstError {
    #[error("Could not unwrap Lexer Literal Value as a String: {0:?}")]
    UnwrapString(Option<lexer::LiteralValue>),
    #[error("Could not unwrap Lexer Literal Value as an f64: {0:?}")]
    UnwrapFloat(Option<lexer::LiteralValue>),
    #[error("Cannot compare against type '{0}'")]
    CompareTypes(String),
    // #[error("Could not unwrap Lexer Literal Value as an usize: {0:?}")]
    // UnwrapInt(Option<lexer::LiteralValue>),
    #[error("Could not create literal value from token: {0:?}")]
    LiteralValueCreate(Token),
    #[error("{0:?} not implemented for {1}")]
    Unimplemented(TokenType, LiteralValue),
    #[error("Unreacheble at evaluating expression: {0}")]
    Unreachable(String),
    #[error("Invalid operator: {0:?}")]
    InvalidOperator(TokenType),
    #[error("Invalid operation: {0:?} {1} {2:?}")]
    InvalidOperation(LiteralValue, String, LiteralValue),
    #[error("Cannot divide by 0: {0}/{1}")]
    Division0(LiteralValue, LiteralValue),
}

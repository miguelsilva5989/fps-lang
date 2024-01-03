use crate::lexer::{self, Token};
use std::fmt::{self, Display, Formatter};

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
enum AstError {
    #[error("Could not unwrap Lexer Literal Value as a String: {0:?}")]
    UnwrapString(Option<lexer::LiteralValue>),
    #[error("Could not unwrap Lexer Literal Value as an i64: {0:?}")]
    UnwrapInt(Option<lexer::LiteralValue>),
    #[error("Could not create literal value from token: {0:?}")]
    LiteralValueCreate(Token),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum LiteralValue {
    Number(i64),
    StringValue(String),
    True,
    False,
}

impl Display for LiteralValue {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        match self {
            LiteralValue::Number(val) => write!(format, "{}", val.to_string()),
            LiteralValue::StringValue(val) => write!(format, "{}", val),
            LiteralValue::True => write!(format, "true"),
            LiteralValue::False => write!(format, "false"),
        }
    }
}

fn unwrap_as_string(literal: Option<lexer::LiteralValue>) -> Result<String> {
    match literal {
        Some(lexer::LiteralValue::StringValue(s)) => Ok(s.clone()),
        Some(lexer::LiteralValue::Identifier(s)) => Ok(s.clone()),
        _ => return Err(AstError::UnwrapString(literal).into()),
    }
}

fn unwrap_as_i64(literal: Option<lexer::LiteralValue>) -> Result<i64> {
    match literal {
        Some(lexer::LiteralValue::Int(s)) => Ok(s),
        _ => return Err(AstError::UnwrapInt(literal).into()),
    }
}

impl LiteralValue {
    pub fn from_token(token: Token) -> Result<Self> {
        use crate::lexer::TokenType::*;
        match token.token_type {
            StringLiteral => Ok(LiteralValue::StringValue(unwrap_as_string(token.literal)?)),
            Number => Ok(LiteralValue::Number(unwrap_as_i64(token.literal)?)),
            _ => return Err(AstError::LiteralValueCreate(token).into()),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expr: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Display for Expr {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        match self {
            Expr::Binary { left, operator, right } => write!(format, "({} {} {})", operator.lexeme, left, right),
            Expr::Grouping { expr } => write!(format, "(group {})", expr),
            Expr::Literal { value } => write!(format, "{}", value),
            Expr::Unary { operator, right } => write!(format, "({} {})", operator.lexeme, right),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::TokenType;

    use super::*;

    #[test]
    fn pretty_print_ast() {
        use Expr::*;

        let minus_token = Token::new(TokenType::Minus, "-".to_string(), None, 0, 0);
        let num = Literal {
            value: LiteralValue::Number(123),
        };

        let group = Grouping {
            expr: Box::new(Literal {
                value: LiteralValue::Number(45),
            }),
        };
        let multi = Token::new(TokenType::Star, "*".to_string(), None, 0, 0);

        let ast = Binary {
            left: Box::new(Unary {
                operator: minus_token,
                right: Box::new(num),
            }),
            operator: multi,
            right: Box::new(group),
        };

        assert_eq!(ast.to_string(), "(* (- 123) (group 45))".to_string());
    }
}

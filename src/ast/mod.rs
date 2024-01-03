use crate::lexer::{self, Token, TokenType};
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
    #[error("{0:?} not implemented for {1}")]
    Unimplemented(TokenType, LiteralValue),
    #[error("Unreacheble")]
    Unreachable,
}

#[derive(Debug, Clone)]
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

impl LiteralValue {
    pub fn from_token(token: Token) -> Result<Self> {
        use TokenType::*;
        match token.token_type {
            StringLiteral => Ok(Self::StringValue(unwrap_as_string(token.literal)?)),
            Number => Ok(Self::Number(unwrap_as_i64(token.literal)?)),
            True => Ok(Self::True),
            False => Ok(Self::False),
            _ => return Err(AstError::LiteralValueCreate(token).into()),
        }
    }

    fn is_falsy(&self) -> LiteralValue {
        use LiteralValue::*;
        match self {
            Number(num) => {
                if *num == 0 {
                    LiteralValue::False
                } else {
                    LiteralValue::True
                }
            }
            StringValue(val) => {
                if val.len() == 0 {
                    LiteralValue::True
                } else {
                    LiteralValue::False
                }
            }
            True => LiteralValue::False,
            False => LiteralValue::True,
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

impl Expr {
    pub fn evaluate(&self) -> Result<LiteralValue> {
        match self {
            Expr::Grouping { expr } => expr.evaluate(),
            Expr::Literal { value } => Ok((*value).clone()),
            Expr::Unary { operator, right } => {
                let rhs = right.evaluate()?;
                
                let result = match (&rhs, operator.token_type) {
                    (LiteralValue::Number(num), TokenType::Minus) => Ok(LiteralValue::Number(-num)),
                    (_, TokenType::Minus) => Err(AstError::Unimplemented(TokenType::Minus, rhs).into()),
                    (any, TokenType::Bang) => Ok(any.is_falsy()),
                    _ => Err(AstError::Unreachable.into()),
                };
                
                result
            }
            Expr::Binary { left, operator, right } => todo!(),
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

use std::ops::{Add, Div, Mul, Sub};
use std::fmt::{self, Display, Formatter};
use anyhow::Result;

use crate::lexer::{self, Token, TokenType};
use super::AstError;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LiteralValue {
    Number(f64),
    StringValue(String),
    Boolean(bool),
}

impl Display for LiteralValue {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        match self {
            LiteralValue::Number(val) => write!(format, "{}", val.to_string()),
            LiteralValue::StringValue(val) => write!(format, "{}", val),
            LiteralValue::Boolean(val) => match val {
                true => write!(format, "true"),
                false => write!(format, "false"),
            },
        }
    }
}

impl Into<f64> for LiteralValue {
    fn into(self) -> f64 {
        match self {
            LiteralValue::Number(x) => x,
            LiteralValue::Boolean(_) => panic!("Bool cannot be cast into f64"),
            _ => todo!(),
        }
    }
}

impl Add for LiteralValue {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self {
            LiteralValue::Number(x) => LiteralValue::Number(x + <LiteralValue as Into<f64>>::into(other)),
            LiteralValue::Boolean(val) => panic!("Cannot Add bool '{val}' with number"),
            _ => todo!(),
        }
    }
}
impl Sub for LiteralValue {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match self {
            LiteralValue::Number(x) => LiteralValue::Number(x - <LiteralValue as Into<f64>>::into(other)),
            LiteralValue::Boolean(val) => panic!("Cannot Subtract bool '{val}' with number"),
            _ => todo!(),
        }
    }
}
impl Mul for LiteralValue {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match self {
            LiteralValue::Number(x) => LiteralValue::Number(x * <LiteralValue as Into<f64>>::into(other)),
            LiteralValue::Boolean(val) => panic!("Cannot Multiply bool '{val}' with number"),
            _ => todo!(),
        }
    }
}
impl Div for LiteralValue {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match self {
            LiteralValue::Number(x) => LiteralValue::Number(x / <LiteralValue as Into<f64>>::into(other)),
            LiteralValue::Boolean(val) => panic!("Cannot Divide bool '{val}' with number"),
            _ => todo!(),
        }
    }
}

impl LiteralValue {
    pub fn from_token(token: Token) -> Result<Self> {
        use TokenType::*;
        match token.token_type {
            StringLiteral => Ok(Self::StringValue(unwrap_as_string(token.literal)?)),
            Number => Ok(Self::Number(unwrap_as_f64(token.literal)?)),
            True => Ok(Self::Boolean(true)),
            False => Ok(Self::Boolean(false)),
            _ => return Err(AstError::LiteralValueCreate(token).into()),
        }
    }

    pub fn is_falsy(&self) -> LiteralValue {
        use LiteralValue::*;
        match self {
            Number(num) => {
                if *num == 0. {
                    LiteralValue::Boolean(false)
                } else {
                    LiteralValue::Boolean(true)
                }
            }
            StringValue(val) => {
                if val.len() == 0 {
                    LiteralValue::Boolean(true)
                } else {
                    LiteralValue::Boolean(false)
                }
            }
            Boolean(val) => LiteralValue::Boolean(!*val),
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

fn unwrap_as_f64(literal: Option<lexer::LiteralValue>) -> Result<f64> {
    match literal {
        Some(lexer::LiteralValue::Float(s)) => Ok(s),
        _ => return Err(AstError::UnwrapInt(literal).into()),
    }
}

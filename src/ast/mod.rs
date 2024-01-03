use crate::lexer::{self, Token, TokenType};
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
enum AstError {
    #[error("Could not unwrap Lexer Literal Value as a String: {0:?}")]
    UnwrapString(Option<lexer::LiteralValue>),
    #[error("Could not unwrap Lexer Literal Value as an f64: {0:?}")]
    UnwrapInt(Option<lexer::LiteralValue>),
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

    fn is_falsy(&self) -> LiteralValue {
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
    fn evaluate_numeric_arithmetic_expression(&self, left: LiteralValue, right: LiteralValue, operator: &Token) -> Result<LiteralValue> {
        match operator.token_type {
            TokenType::Plus => Ok(left + right),
            TokenType::Minus => Ok(left - right),
            TokenType::Star => Ok(left * right),
            TokenType::Slash => {
                if right == LiteralValue::Number(0.) {
                    return Err(AstError::Division0(left, right).into());
                }

                Ok(left / right)
            }
            TokenType::Greater => Ok(self::LiteralValue::Boolean(left > right)),
            TokenType::GreaterEqual => Ok(self::LiteralValue::Boolean(left >= right)),
            TokenType::Less => Ok(self::LiteralValue::Boolean(left < right)),
            TokenType::LessEqual => Ok(self::LiteralValue::Boolean(left <= right)),
            TokenType::BangEqual => Ok(self::LiteralValue::Boolean(left != right)),
            TokenType::EqualEqual => Ok(self::LiteralValue::Boolean(left == right)),

            _ => Err(AstError::InvalidOperator(operator.token_type).into()),
        }
    }

    fn evaluate_string_expression(&self, left: LiteralValue, right: LiteralValue, operator: &Token) -> Result<LiteralValue> {
        match operator.token_type {
            TokenType::Greater => Ok(self::LiteralValue::Boolean(left > right)),
            TokenType::GreaterEqual => Ok(self::LiteralValue::Boolean(left >= right)),
            TokenType::Less => Ok(self::LiteralValue::Boolean(left < right)),
            TokenType::LessEqual => Ok(self::LiteralValue::Boolean(left <= right)),
            TokenType::BangEqual => Ok(self::LiteralValue::Boolean(left != right)),
            TokenType::EqualEqual => Ok(self::LiteralValue::Boolean(left == right)),

            _ => Err(AstError::InvalidOperator(operator.token_type).into()),
        }
    }

    fn evaluate_bool_expression(&self, left: LiteralValue, right: LiteralValue, operator: &Token) -> Result<LiteralValue> {
        match operator.token_type {
            TokenType::BangEqual => Ok(self::LiteralValue::Boolean(left != right)),
            TokenType::EqualEqual => Ok(self::LiteralValue::Boolean(left == right)),

            _ => Err(AstError::InvalidOperator(operator.token_type).into()),
        }
    }

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
                    _ => Err(AstError::Unreachable(self.to_string()).into()),
                };

                result
            }
            Expr::Binary { left, operator, right } => {
                let lhs = left.evaluate()?;
                let rhs = right.evaluate()?;

                if matches!(lhs, LiteralValue::Number(_)) && matches!(rhs, LiteralValue::Number(_)) {
                    return self.evaluate_numeric_arithmetic_expression(lhs, rhs, operator);
                } else if matches!(lhs, LiteralValue::StringValue(_)) && matches!(rhs, LiteralValue::StringValue(_)) {
                    return self.evaluate_string_expression(lhs, rhs, operator);
                } else if matches!(lhs, LiteralValue::Boolean(_)) && matches!(rhs, LiteralValue::Boolean(_)) {
                    return self.evaluate_bool_expression(lhs, rhs, operator);
                } else {
                    return Err(AstError::InvalidOperation(lhs, operator.lexeme.clone(), rhs).into());
                }
            }
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
            value: LiteralValue::Number(123.),
        };

        let group = Grouping {
            expr: Box::new(Literal {
                value: LiteralValue::Number(45.),
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

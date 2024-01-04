use anyhow::Result;
use std::fmt::{self, Display, Formatter};

use super::literal::LiteralValue;
use super::AstError;
use crate::lexer::{Token, TokenType};

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

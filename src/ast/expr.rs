use anyhow::Result;
use std::fmt::{self, Display, Formatter};

use super::AstError;
use super::{environment::Environment, literal::LiteralValue};
use crate::lexer::{Token, TokenType};

#[derive(Debug, Clone, PartialEq)]
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
    ReservedLiteral {
        value: String,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        id: Token,
    },
    Assign {
        id: Token,
        value: Box<Expr>,
    },
    Ignore {
        token: Token,
    },
    // Fps {
    //     count: f64,
    // },
}

impl Display for Expr {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        match self {
            Expr::Binary { left, operator, right } => write!(format, "({} {} {})", operator.lexeme, left, right),
            Expr::Grouping { expr } => write!(format, "(group {})", expr),
            Expr::Literal { value } => write!(format, "{}", value),
            Expr::ReservedLiteral { value } => write!(format, "{}", value),
            Expr::Logical { left, operator, right } => write!(format, "({} {} {})", operator.lexeme, left, right),
            Expr::Unary { operator, right } => write!(format, "({} {})", operator.lexeme, right),
            Expr::Variable { id } => write!(format, "(var {})", id.lexeme),
            Expr::Assign { id, value } => write!(format, "({} = {})", id.lexeme, value),
            Expr::Ignore { token } => write!(format, "(ignored Token {})", token),
            // Expr::Fps { count } => write!(format, "FPS count {}", count),
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

    pub fn eval(&self, environment: &mut Environment) -> Result<LiteralValue> {
        match self {
            Expr::Variable { id } => {
                let val = environment.get(id.lexeme.to_owned())?;
                Ok(val)
            }
            Expr::Assign { id, value } => {
                // environment.get(id.lexeme.to_owned())?;
                let value = value.eval(&mut *environment)?;
                environment.assign(id.lexeme.to_owned(), value)?;
                Ok(environment.get(id.lexeme.to_owned())?)
            }
            Expr::Grouping { expr } => expr.eval(environment),
            Expr::Literal { value } => Ok((*value).clone()),
            Expr::Logical { left, operator, right } => {
                match operator.token_type {
                    TokenType::Or => {
                        let left_val = left.eval(environment)?;
                        let bool_left = left_val.is_true()?;

                        if bool_left == LiteralValue::Boolean(true) {
                            Ok(left_val)
                        } else {
                            right.eval(environment)
                        }
                    },
                    TokenType::And => {
                        let left_val = left.eval(environment)?;
                        let bool_left = left_val.is_true()?;

                        if bool_left == LiteralValue::Boolean(false) {
                            Ok(left_val)
                        } else {
                            right.eval(environment)
                        }
                    },
                    _ => Err(AstError::InvalidOperator(operator.token_type).into()),
                }
            },
            Expr::Unary { operator, right } => {
                let rhs = right.eval(environment)?;

                let result = match (&rhs, operator.token_type) {
                    (LiteralValue::Number(num), TokenType::Minus) => Ok(LiteralValue::Number(-num)),
                    (_, TokenType::Minus) => Err(AstError::Unimplemented(TokenType::Minus, rhs).into()),
                    (any, TokenType::Bang) => Ok(any.is_false()?),
                    _ => Err(AstError::Unreachable(self.to_string()).into()),
                };

                result
            }
            Expr::Binary { left, operator, right } => {
                let lhs = left.eval(environment)?;
                let rhs = right.eval(environment)?;

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
            Expr::Ignore { token: _ } => Ok(LiteralValue::Null),
            Expr::ReservedLiteral { value } => {
                println!("TODO implement IT inside for loop. {}", value);
                todo!()
            },
            // Expr::Fps { count } => Ok(LiteralValue::Number(*count)),
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

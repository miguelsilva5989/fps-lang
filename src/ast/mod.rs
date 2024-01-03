use crate::lexer::Token;
use std::fmt::{self, Debug, Display, Formatter};

pub enum LiteralValue {
    Number(f64),
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
            value: LiteralValue::Number(123.0),
        };

        let group = Grouping {
            expr: Box::new(Literal {
                value: LiteralValue::Number(45.67),
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

        assert_eq!(ast.to_string(), "(* (- 123) (group 45.67))".to_string());
    }
}

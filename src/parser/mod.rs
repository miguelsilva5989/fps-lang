use crate::{
    ast::{Expr, LiteralValue},
    lexer::{Token, TokenType},
};

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
enum ParserError {
    #[error("Could not consume: {}", {0})]
    Consume(String),
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
        use TokenType::*;
        let mut expr: Expr = self.comparison()?;

        while self.match_tokens(vec![BangEqual, EqualEqual]) {
            let operator = self.previous();
            let rhs = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    fn match_token(&mut self, tt: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek().token_type == tt {
            self.advance();
            return true;
        }

        false
    }

    fn match_tokens(&mut self, token_types: Vec<TokenType>) -> bool {
        for tt in token_types {
            if self.match_token(tt) {
                return true;
            }
        }
        false
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    /// try to recover from parsing errors
    fn synchronize(&mut self) {
        use TokenType::*;
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == Semicolon {
                return;
            }

            match self.peek().token_type {
                For | Print | Println => return,
                _ => ()
            }

            self.advance();
        }
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<()> {
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
        }
        return Err(ParserError::Consume(msg.to_string()).into());
    }

    fn primary(&mut self) -> Result<Expr> {
        use TokenType::*;

        if self.match_token(OpenParen) {
            let expr = self.expression()?;
            self.consume(CloseParen, "Expected ')'")?;
            Ok(Expr::Grouping { expr: Box::new(expr) })
        } else {
            let token = self.peek();
            self.advance();
            Ok(Expr::Literal {
                value: LiteralValue::from_token(token)?,
            })
        }
    }

    fn unary(&mut self) -> Result<Expr> {
        use TokenType::*;
        if self.match_tokens(vec![Bang, Minus]) {
            let operator = self.previous();
            let rhs = self.unary()?;
            Ok(Expr::Unary {
                operator,
                right: Box::new(rhs),
            })
        } else {
            self.primary()
        }
    }

    fn fac(&mut self) -> Result<Expr> {
        use TokenType::*;
        let mut expr = self.unary()?;

        while self.match_tokens(vec![Slash, Star]) {
            let operator = self.previous();
            let rhs = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(rhs),
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        use TokenType::*;
        let mut expr = self.fac()?;

        while self.match_tokens(vec![Minus, Plus]) {
            let operator = self.previous();
            let rhs = self.fac()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(rhs),
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        use TokenType::*;
        let mut expr = self.term()?;

        while self.match_tokens(vec![Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous();
            let rhs = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(rhs),
            };
        }
        Ok(expr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{LiteralValue::*, Token, TokenType::*, FpsInput};

    macro_rules! token {
        ($token_type: expr, $lexeme: expr, $literal: expr) => {
            Token::new($token_type, $lexeme.into(), $literal, 0, 0)
        };
    }

    #[test]
    fn test_addition() {
        //4+20;
        let input = vec![
            token!(Number, "4", Some(Int(4))),
            token!(Plus, "+", None),
            token!(Number, "20", Some(Int(20))),
            token!(Semicolon, ";", None),
        ];

        let mut parser = Parser::new(input);
        let expression = parser.expression();

        assert_eq!(expression.unwrap().to_string(), "(+ 4 20)")
    }

    #[test]
    fn test_comparison() {
        let input = "4 + 20 == 5 + 6";
        let mut scanner = FpsInput::new(input);
        scanner.scan_tokens().expect("error scanning tokens");

        let mut parser = Parser::new(scanner.tokens);
        let expression = parser.expression();

        assert_eq!(expression.unwrap().to_string(), "(== (+ 4 20) (+ 5 6))")
    }
}

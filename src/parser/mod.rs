use crate::{
    ast::{literal::LiteralValue, arithmetic::Expr},
    lexer::{Token, TokenType, KEYWORDS},
};

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
enum ParserError {
    #[error("Could not consume: '{0:?}'")]
    Consume(String),
    #[error("Expected expression.")]
    ExpectedExpression,
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

    pub fn parse(&mut self) -> Result<Expr> {
        let expr = self.expression();

        if !self.is_at_end() {
            if !self.match_token(TokenType::Semicolon) {
                println!("{:?}", self);
                println!("{:?}", expr);

                println!("{:?}", self.is_at_end());
                println!("{:?}", self.peek());

                panic!("to fix remaining input. ex:   ( 1 )  + 2    only translates to  (group 1)");
                todo!();
            }
        }

        expr
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
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

            // TokenType keywords
            if KEYWORDS.get(&*self.peek().lexeme).is_some() {
                return;
            }

            self.advance();
        }
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<()> {
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
            return Ok(());
        }
        return Err(ParserError::Consume(msg.to_string()).into());
    }

    fn primary(&mut self) -> Result<Expr> {
        use TokenType::*;

        let token = self.peek();
        let result = match token.token_type {
            Number | StringLiteral | True | False => Ok(Expr::Literal {
                value: LiteralValue::from_token(token)?,
            }),
            OpenParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(CloseParen, "Expected ')' after expression.")?;
                Ok(Expr::Grouping { expr: Box::new(expr) })
            }
            _ => return Err(ParserError::ExpectedExpression.into()),
        };

        // match token and we can advance pointer
        self.advance();

        result
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{FpsInput, LiteralValue::*, Token, TokenType::*};

    macro_rules! token {
        ($token_type: expr, $lexeme: expr, $literal: expr) => {
            Token::new($token_type, $lexeme.into(), $literal, 0, 0)
        };
    }

    #[test]
    fn test_addition() {
        //4+20;
        let input = vec![
            token!(Number, "4", Some(Float(4.))),
            token!(Plus, "+", None),
            token!(Number, "20", Some(Float(20.))),
            token!(Semicolon, ";", None),
        ];

        let mut parser = Parser::new(input);
        let expression = parser.parse();

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

    #[test]
    fn test_comparison_paren() {
        let input = "1 == (2 - 1)";
        let mut scanner = FpsInput::new(input);
        scanner.scan_tokens().expect("error scanning tokens");

        let mut parser = Parser::new(scanner.tokens);
        let expression = parser.expression();

        assert_eq!(expression.unwrap().to_string(), "(== 1 (group (- 2 1)))")
    }
}

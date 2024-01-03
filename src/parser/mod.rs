use crate::{
    ast::Expr,
    lexer::{Token, TokenType},
};

use anyhow::Result;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression(&mut self) {
        self.equality();
    }

    fn equality(&mut self) -> Expr {
        use TokenType::*;
        let mut expr: Expr = self.comparison();

        while self.match_tokens(&[BangEqual, EqualEqual]) {
            let operator = self.previous();
            let rhs = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(rhs),
            };
        }

        expr
    }

    fn match_token(&mut self, tt: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        if &self.peek().token_type == tt {
            self.advance();
            return true
        }

        false
    }

    fn match_tokens(&mut self, token_types: &[TokenType]) -> bool {
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

    fn comparison(&mut self) -> Expr {
        todo!()
    }
}

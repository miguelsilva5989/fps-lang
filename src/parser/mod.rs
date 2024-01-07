use crate::{
    ast::{expr::Expr, literal::LiteralValue, statement::Statement},
    lexer::{Token, TokenType, KEYWORDS},
};

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
enum ParserError {
    #[error("Could not consume: '{0:?}'")]
    Consume(String),
    #[error("Expected expression for token: '{0}' at line {1}")]
    ExpectedExpression(String, usize),
    #[error("Expected FPS End token '##' at the end of the file")]
    ExpectedFpsEnd,
    #[error("Invalid variable declaration: '{0}'")]
    Declaration(String),
    #[error("Invalid assignment target")]
    InvalidAssignment,
    #[error("Errors parsing: {0:?}")]
    MultipleErrors(Vec<String>),
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    // current_fps: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>> {
        let mut statements: Vec<Statement> = vec![];
        let mut errors: Vec<String> = vec![];

        while !self.is_at_end() {
            let statement = self.declaration();
            match statement {
                Ok(s) => statements.push(s),
                Err(err) => {
                    errors.push(err.to_string());
                    self.synchronize();
                }
            }
        }

        if self.previous().token_type != TokenType::FpsEnd {
            return Err(ParserError::ExpectedFpsEnd.into());
        }

        if errors.len() > 0 {
            return Err(ParserError::MultipleErrors(errors).into());
        }
        Ok(statements)

        // let expr = self.expression();

        // if !self.is_at_end() {
        //     if !self.match_token(TokenType::Semicolon) {
        //         println!("{:?}", self);
        //         println!("{:?}", expr);

        //         println!("{:?}", self.is_at_end());
        //         println!("{:?}", self.peek());

        //         panic!("to fix remaining input. ex:   ( 1 )  + 2    only translates to  (group 1)");
        //         todo!();
        //     }
        // }

        // expr
    }

    fn declaration(&mut self) -> Result<Statement> {
        if self.match_token(TokenType::Declaration) {
            match self.declaration_statement() {
                Ok(s) => Ok(s),
                Err(err) => {
                    self.synchronize();
                    return Err(ParserError::Declaration(err.to_string()).into());
                }
            }
        // } else if self.match_token(TokenType::Fps) {
        //     let token = self.peek();
        //     // self.current_fps += token.fps;
        //     Ok(Statement::Fps(token))
        } else {
            self.statement()
        }
    }

    fn statement(&mut self) -> Result<Statement> {
        use TokenType::*;

        // if self.match_token(Print) {
        //     self.print_statement()
        // }else if self.match_token(OpenBrace) {
        //     self.block_statement()
        // } else if self.match_tokens(vec![Fps, FpsEnd]) {
        //     Ok(Statement::Fps(self.previous()))
        // } else {
        //     self.expression_statement()
        // }

        match self.peek().token_type {
            Comment => {
                self.advance();
                Ok(Statement::Comment(self.previous()))
            }
            Fps => {
                self.advance();
                Ok(Statement::Fps(self.previous()))
            }
            FpsEnd => {
                self.advance();
                Ok(Statement::FpsEnd(self.previous()))
            }
            Print => {
                self.advance();
                self.print_statement()
            }
            OpenBrace => {
                self.advance();
                self.block_statement()
            }
            If => {
                self.advance();
                self.if_statement()
            }
            For => {
                self.advance();
                self.for_statement()
            }
            _ => self.expression_statement(),
        }
    }

    fn print_statement(&mut self) -> Result<Statement> {
        self.consume(TokenType::OpenParen, "Expected '('")?;
        let mut expressions = self.eval_until(TokenType::CloseParen)?;

        if expressions.len() == 0 {
            todo!("return error")
        }

        self.consume(TokenType::Semicolon, "Expected ';' after statement")?;

        Ok(Statement::Print(expressions.remove(0)))
    }

    fn block_statement(&mut self) -> Result<Statement> {
        let mut statements: Vec<Statement> = vec![];

        while !self.check_next_token(TokenType::CloseBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::CloseBrace, "Expected '}' after block")?;

        Ok(Statement::Block { statements })
    }

    fn if_statement(&mut self) -> Result<Statement> {
        let expr = self.expression()?;

        self.consume(TokenType::OpenBrace, "Expected '{' after if condition")?;
        let mut then_block: Vec<Statement> = vec![];
        while !self.check_next_token(TokenType::CloseBrace) && !self.is_at_end() {
            then_block.push(self.declaration()?);
        }
        self.consume(TokenType::CloseBrace, "Expected '}' after if then block")?;

        let mut else_block = None;
        if self.match_token(TokenType::Else) {
            self.consume(TokenType::OpenBrace, "Expected '{' after else keyword")?;
            let mut else_block_statements: Vec<Statement> = vec![];
            while !self.check_next_token(TokenType::CloseBrace) && !self.is_at_end() {
                else_block_statements.push(self.declaration()?);
            }
            self.consume(TokenType::CloseBrace, "Expected '}' after else block")?;
            else_block = Some(else_block_statements)
        }

        Ok(Statement::If {
            condition: expr,
            then_block: then_block,
            else_block: else_block,
        })
    }

    fn for_statement(&mut self) -> Result<Statement> {
        use TokenType::*;
        if self.check_next_token(Range) || self.check_next_token(RangeEqual) {
            let expr = self.expression()?;

            self.consume(TokenType::OpenBrace, "Expected '{' after for range")?;
            let mut for_block: Vec<Statement> = vec![];
            while !self.check_next_token(TokenType::CloseBrace) && !self.is_at_end() {
                for_block.push(self.declaration()?);
            }
            self.consume(TokenType::CloseBrace, "Expected '}' after for block")?;

            Ok(Statement::For {
                range: expr,
                for_block: for_block,
            })
        } else {
            return Err(ParserError::Consume("Expected a Range/RangeEqual after for declaraiton".to_owned()).into());
        }
    }

    fn expression_statement(&mut self) -> Result<Statement> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after statement")?;
        Ok(Statement::ArithmeticExpr(expr))
    }

    fn declaration_statement(&mut self) -> Result<Statement> {
        use TokenType::*;
        let token = self.consume(Identifer, "Expected variable name")?;

        let expr = if self.match_token(Equal) {
            self.expression()?
        } else {
            Expr::Literal { value: LiteralValue::Null }
        };

        self.consume(Semicolon, "Expected ';' after declaration")?;

        Ok(Statement::Declaration { id: token, expr })
    }

    fn assignment(&mut self) -> Result<Expr> {
        let expr = self.equality()?;

        // println!("expr {:?}", expr);

        if self.match_token(TokenType::Equal) {
            let val = self.assignment()?;

            match expr {
                Expr::Variable { id } => return Ok(Expr::Assign { id, value: Box::new(val) }),
                _ => return Err(ParserError::InvalidAssignment.into()),
            }
        }

        Ok(expr)
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn check_next_token(&mut self, tt: TokenType) -> bool {
        self.peek().token_type == tt
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

    fn eval_until(&mut self, token_type: TokenType) -> Result<Vec<Expr>> {
        let mut consumed: Vec<Expr> = vec![];

        loop {
            consumed.push(self.expression()?);
            if self.is_at_end() {
                return Err(ParserError::Consume(format!("Expected '{:?}'", token_type).to_string()).into());
            }
            if self.peek().token_type == token_type {
                self.consume(token_type, "Expected ')'")?;
                break;
            }
        }

        Ok(consumed)
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token> {
        let token = self.peek();
        if token.token_type == token_type {
            self.advance();
            return Ok(self.previous());
        }
        return Err(ParserError::Consume(msg.to_string()).into());
    }

    fn primary(&mut self) -> Result<Expr> {
        use TokenType::*;

        let token = self.peek();
        // println!("{}", token);
        let result = match token.token_type {
            Number | StringLiteral | True | False | Null => {
                self.advance();
                Ok(Expr::Literal {
                    value: LiteralValue::from_token(token)?,
                })
            }
            Range | RangeEqual => {
                self.advance();
                Ok(Expr::Literal {
                    value: LiteralValue::from_token(token)?,
                })
            }
            OpenParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(CloseParen, "Expected ')' after expression.")?;
                Ok(Expr::Grouping { expr: Box::new(expr) })
            }
            Identifer => {
                self.advance();
                let id = self.previous();
                Ok(Expr::Variable { id })
            }
            Fps => Ok(Expr::Literal {
                value: LiteralValue::from_token(token)?,
            }),
            FpsEnd => Ok(Expr::Literal { value: LiteralValue::Null }),
            Comment => Ok(Expr::Ignore { token: token }),
            It => {
                self.advance();
                Ok(Expr::ReservedLiteral { value: token.lexeme })
            }
            _ => {
                // println!("{:?}", token);
                return Err(ParserError::ExpectedExpression(token.lexeme.to_owned(), token.line).into());
            }
        };

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
            token!(Number, "4.", Some(Float(4.))),
            token!(Plus, "+", None),
            token!(Number, "20", Some(Float(20.))),
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

    #[test]
    fn test_comparison_paren() {
        let input = "1 == (2 - 1)";
        let mut scanner = FpsInput::new(input);
        scanner.scan_tokens().expect("error scanning tokens");

        let mut parser = Parser::new(scanner.tokens);
        let expression = parser.expression();

        assert_eq!(expression.unwrap().to_string(), "(== 1 (group (- 2 1)))")
    }

    #[test]
    fn test_print_statement() {
        use crate::ast::expr::*;
        let input = "print(1); ##";
        let mut scanner = FpsInput::new(input);
        scanner.scan_tokens().expect("error scanning tokens");

        let mut parser = Parser::new(scanner.tokens);
        let expression = parser.parse();

        let expected = vec![
            Statement::Print(Expr::Literal {
                value: LiteralValue::Number(1.),
            }),
            Statement::FpsEnd(Token {
                token_type: FpsEnd,
                lexeme: "##".to_owned(),
                literal: None,
                line: 1,
                pos: 12,
            }),
        ];

        assert_eq!(expression.unwrap(), expected)
    }

    #[test]
    fn declaration() {
        use crate::ast::expr::*;
        let input = "let a = 1.; ##";
        let mut scanner = FpsInput::new(input);
        scanner.scan_tokens().expect("error scanning tokens");

        let mut parser = Parser::new(scanner.tokens);
        let expression = parser.parse();

        let expected = vec![
            Statement::Declaration {
                id: Token {
                    token_type: Identifer,
                    lexeme: "a".to_owned(),
                    literal: Some(Identifier("a".to_owned())),
                    line: 1,
                    pos: 5,
                },
                expr: Expr::Literal {
                    value: LiteralValue::Number(1.),
                },
            },
            Statement::FpsEnd(Token {
                token_type: FpsEnd,
                lexeme: "##".to_owned(),
                literal: None,
                line: 1,
                pos: 14,
            }),
        ];

        assert_eq!(expression.unwrap(), expected)
    }

    #[test]
    fn declaration_assign() {
        use crate::ast::expr::*;
        let input = "let a = 1;a = 2; ##";
        let mut scanner = FpsInput::new(input);
        scanner.scan_tokens().expect("error scanning tokens");

        let mut parser = Parser::new(scanner.tokens);
        let expression = parser.parse();

        let expected = vec![
            Statement::Declaration {
                id: Token {
                    token_type: Identifer,
                    lexeme: "a".to_owned(),
                    literal: Some(Identifier("a".to_owned())),
                    line: 1,
                    pos: 5,
                },
                expr: Expr::Literal {
                    value: LiteralValue::Number(1.),
                },
            },
            Statement::ArithmeticExpr(Expr::Assign {
                id: Token {
                    token_type: Identifer,
                    lexeme: "a".to_string(),
                    literal: Some(Identifier("a".to_string())),
                    line: 1,
                    pos: 11,
                },
                value: Box::from(Expr::Literal {
                    value: LiteralValue::Number(2.),
                }),
            }),
            Statement::FpsEnd(Token {
                token_type: FpsEnd,
                lexeme: "##".to_owned(),
                literal: None,
                line: 1,
                pos: 19,
            }),
        ];

        assert_eq!(expression.unwrap(), expected)
    }

    #[test]
    fn declaration_print() {
        use crate::ast::expr::*;
        let input = "let a = 1;print(a); ##";
        let mut scanner = FpsInput::new(input);
        scanner.scan_tokens().expect("error scanning tokens");

        let mut parser = Parser::new(scanner.tokens);
        let expression = parser.parse();

        let expected = vec![
            Statement::Declaration {
                id: Token {
                    token_type: Identifer,
                    lexeme: "a".to_owned(),
                    literal: Some(Identifier("a".to_owned())),
                    line: 1,
                    pos: 5,
                },
                expr: Expr::Literal {
                    value: LiteralValue::Number(1.),
                },
            },
            Statement::Print(Expr::Variable {
                id: Token {
                    token_type: Identifer,
                    lexeme: "a".to_owned(),
                    literal: Some(Identifier("a".to_owned())),
                    line: 1,
                    pos: 17,
                },
            }),
            Statement::FpsEnd(Token {
                token_type: FpsEnd,
                lexeme: "##".to_owned(),
                literal: None,
                line: 1,
                pos: 22,
            }),
        ];

        assert_eq!(expression.unwrap(), expected)
    }
}

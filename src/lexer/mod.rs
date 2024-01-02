use std::fmt::{self, Debug, Display, Formatter};

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
enum FpsError {
    #[error("Unrecognized char '{0}' at line {1}")]
    UnrecognizedChar(char, usize),
}

#[derive(Debug)]
pub enum TokenType {
    // single char
    Fps,
    Semicolon,
    Colon,
    Equals,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Plus,
    Minus,
    Star,
    Slash,

    // literals
    Identifer,
    String,
    Number,

    // keywords
    For,
    Print,
    Println,

    // Ignore
    Whitespace,
    Eof,
}

#[derive(Debug)]
pub enum LiteralValue {
    Int(i64),
    Float(f64),
    String(String),
    Identifier(String),
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<LiteralValue>,
    line: usize,
    pos: usize,
}

impl Display for Token {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        write!(
            format,
            "{:?} {} {:?} pos: {}-{}",
            self.token_type, self.lexeme, self.literal, self.line, self.pos
        )
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<LiteralValue>, line: usize, pos: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
            pos,
        }
    }
}

macro_rules! token {
    // single char
    ($token_type: ident, $lexeme: expr, $line: expr, $pos: expr) => {
        Token::new(TokenType::$token_type, $lexeme.into(), None, $line, $pos)
    };
    // literal
    ($token_type: ident, $lexeme: expr, $literal: expr, $line: expr, $pos: expr) => {
        Token::new(TokenType::$token_type, $lexeme.into(), $literal, $line, $pos)
    };
}

pub struct FpsInput<'a> {
    input: &'a str,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'a> FpsInput<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.input.len()
    }

    pub fn scan_tokens(&mut self) -> Result<()> {
        // let lines = self.input.lines();
        // for (line_number, line) in lines.enumerate() {
        //     let mut iter = line.chars().into_iter().peekable();
        // let mut iter = self.input.chars().into_iter().peekable();

        // let mut pos = 0;
        // while let Some(ch) = iter.next() {
        //     pos += 1;
        //     println!("char {ch}");
        //     self.start = self.current;
        //     let token = self.tokenzine(ch, pos)?;
        //     self.tokens.push(token)
        // }

        // let mut pos = 0;
        while !self.is_at_end() {
            // pos += 1;
            self.line += 1;
            self.start = self.current;
            if let Some(token) = self.tokenzine()? {
                self.tokens.push(token)
            } else {
                break;
            }
        }

        self.tokens.push(token!(Eof, "", None, 0, self.input.len()));

        Ok(())
    }

    fn advance(&mut self) -> Result<Option<char>> {
        let ch = self.input.chars().nth(self.current);
        self.current += 1;
        Ok(ch)
    }

    fn tokenzine(&mut self) -> Result<Option<Token>> {
        if let Ok(Some(ch)) = self.advance() {
            let token = match ch {
                '#' => token!(Fps, ch, self.line, self.current),
                ';' => token!(Semicolon, ch, self.line, self.current),
                ':' => token!(Colon, ch, self.line, self.current),
                '=' => token!(Equals, ch, self.line, self.current),
                '(' => token!(OpenParen, ch, self.line, self.current),
                ')' => token!(CloseParen, ch, self.line, self.current),
                '{' => token!(OpenBrace, ch, self.line, self.current),
                '}' => token!(CloseBrace, ch, self.line, self.current),
                // operations
                '+' => token!(Plus, ch, self.line, self.current),
                '-' => token!(Minus, ch, self.line, self.current),
                '*' => token!(Star, ch, self.line, self.current),
                '/' => token!(Slash, ch, self.line, self.current),

                // '+' | '-' | '*' | '/' | '%' => tokens.push(Token::new(ch.into(), TokenType::BinaryOperator)),
                ' ' => token!(Whitespace, ch, self.line, self.current),
                '\n' => token!(Whitespace, ch, self.line, self.current),
                '\r' => token!(Whitespace, ch, self.line, self.current),
                _ => {
                    self.current += 1;
                    return Err(FpsError::UnrecognizedChar(ch, self.line).into());
                } // _ => {
                  //     if ch == ' ' || ch == '\n' || ch == '\r' || ch == '\t' {
                  //         // ignore whitespaces
                  //     } else if ch.is_numeric() {
                  //         let mut num: String = ch.into();
                  //         while let Some(next) = iter.peek() {
                  //             if next.is_numeric() {
                  //                 num.push(iter.next().unwrap());
                  //             } else {
                  //                 break;
                  //             }
                  //         }
                  //         tokens.push(Token::new(num, TokenType::Number));
                  //     } else if ch.is_alphabetic() {
                  //         let mut id: String = ch.into();
                  //         while let Some(next) = iter.peek() {
                  //             if next.is_alphanumeric() || next == &'_' {
                  //                 id.push(iter.next().unwrap());
                  //             } else {
                  //                 break;
                  //             }
                  //         }

                  //         if RESERVED.contains(&id.as_str()) {
                  //             match id.as_str() {
                  //                 "None" => tokens.push(Token::new(id, TokenType::None)),
                  //                 _ => panic!("need to implement reserverd '{}' keywork in tokenizer", id),
                  //             }
                  //         } else {
                  //             tokens.push(Token::new(id, TokenType::Identifier));
                  //         }
                  //     } else {
                  //         panic!("Token '{}' is not yet implemented", ch);
                  //     }
                  // }
            };

            return Ok(Some(token));
        }
        Ok(None)
    }
}

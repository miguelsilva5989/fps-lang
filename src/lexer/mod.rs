use std::fmt::{self, Debug, Display, Formatter};

use anyhow::Result;

#[derive(Debug)]
pub enum TokenType {
    // single char
    Fps,
    Semicolon,
    Colon,
    Equals,
    OpenParen,
    CloseParen,
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

    // end of file
    Eof,
}
impl Display for TokenType {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        write!(format, "{}", self)
    }
}

#[derive(Debug)]
pub enum LiteralValue {
    Int(i64),
    Float(f64),
    String(String),
    Identifier(String),
}
impl Display for LiteralValue {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        write!(format, "{}", self)
    }
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
            "{} {} {:?} pos: {}-{}",
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

    pub fn scan_tokens(&mut self) -> Result<()> {
        // let lines = self.input.lines();
        // for (line_number, line) in lines.enumerate() {
        //     let mut iter = line.chars().into_iter().peekable();
        let mut iter = self.input.chars().into_iter().peekable();

        while let Some((pos, ch)) = iter.clone().enumerate().next() {
            self.start = self.current;
            self.tokenzine(ch)?;
        }
        // }

        Ok(())
    }

    fn tokenzine(&mut self, ch: char) -> Result<Token>{
        match ch {
            // '(' => self.tokens.push(Token::new(TokenType::OpenParen, ch.into(), None, line_number, pos)),
            '#' => todo!("MAKE A MACRO FOR THE TOKEN NEW func"),
            // ')' => tokens.push(Token::new(ch.into(), TokenType::CloseParen)),
            // '+' | '-' | '*' | '/' | '%' => tokens.push(Token::new(ch.into(), TokenType::BinaryOperator)),
            _ => todo!(), // _ => {
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
        }
    }
}

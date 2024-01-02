use std::fmt::{self, Debug, Display, Formatter};

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
enum FpsError {
    #[error("Unrecognized char '{0}' at line {1}")]
    UnrecognizedChar(char, usize),
    #[error("Unterminated consumption until char '{0:?}' at line {2}. Consumed: {1}")]
    UnterminatedConsumption(Vec<char>, String, usize),
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // single char
    Fps, // #
    Semicolon,
    Equals,
    Colon,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Plus,
    Minus,
    Star,
    Slash,
    // Bang, // !

    // symbols
    Assign, // :=

    // literals
    Identifer,
    String,
    Number,

    // keywords
    For,
    Print,
    Println,

    // Ignore
    Comment,
    Whitespace,
    Eof,
}

#[derive(Debug, PartialEq)]
pub enum LiteralValue {
    Int(i64),
    Float(f64),
    String(String),
    Identifier(String),
}

#[derive(Debug, PartialEq)]
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
        while !self.is_at_end() {
            self.line += 1;
            self.start = self.current;
            if let Some(token) = self.tokenzine()? {
                // ignore whitespaces
                match token.token_type {
                    TokenType::Whitespace => {}
                    _ => self.tokens.push(token),
                }
            } else {
                break;
            }
        }

        self.tokens.push(token!(Eof, "", None, 0, self.input.len()));

        Ok(())
    }

    fn peek(&mut self) -> Result<Option<char>> {
        let ch = self.input.chars().nth(self.current);
        Ok(ch)
    }

    fn eat(&mut self) -> Result<Option<char>> {
        let ch = self.peek();
        self.current += 1;
        ch
    }

    fn consume_until(&mut self, chars: Vec<char>) -> Result<String> {
        let mut consumed = "".to_string();
        loop {
            match self.peek() {
                Ok(is_next) => {
                    if let Some(next) = is_next {
                        if chars.contains(&next) {
                            break;
                        } else {
                            consumed.push_str(next.to_string().as_str());
                            self.current += 1;
                        }
                    } else {
                        break;
                    }
                }
                //Eof
                Err(_) => return Err(FpsError::UnterminatedConsumption(chars, consumed, self.line).into()),
            }
        }
        Ok(consumed)
    }

    fn is_next_char_match(&mut self, ch: char) -> bool {
        match self.peek() {
            Ok(is_next) => {
                if let Some(next) = is_next {
                    if next == ch {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            //Eof
            Err(_) => false,
        }
    }

    fn read_until_eol(&mut self) -> Result<String> {
        self.consume_until(vec!['\n', '\r'])
    }

    fn consume_string(&mut self) -> Result<String> {
        let consumed = self.consume_until(vec!['"']);
        self.current += 1;
        consumed
    }

    fn tokenzine(&mut self) -> Result<Option<Token>> {
        if let Ok(Some(ch)) = self.eat() {
            let token = match ch {
                // whitespaces
                ' ' | '\t' => token!(Whitespace, ch, self.line, self.current),
                '\n' | '\r' => {
                    self.line += 1;
                    token!(Whitespace, ch, self.line, self.current)
                }
                // operations
                '+' => token!(Plus, ch, self.line, self.current),
                '-' => token!(Minus, ch, self.line, self.current),
                '*' => token!(Star, ch, self.line, self.current),
                '/' => {
                    // comments are read until Eol
                    if self.is_next_char_match('/') {
                        self.current += 1;
                        let comment = self.read_until_eol()?;
                        token!(Comment, comment, self.line, self.current)
                    } else {
                        token!(Slash, ch, self.line, self.current)
                    }
                }
                // literals
                '#' => token!(Fps, ch, self.line, self.current),
                ';' => token!(Semicolon, ch, self.line, self.current),
                '=' => token!(Equals, ch, self.line, self.current),
                '(' => token!(OpenParen, ch, self.line, self.current),
                ')' => token!(CloseParen, ch, self.line, self.current),
                '{' => token!(OpenBrace, ch, self.line, self.current),
                '}' => token!(CloseBrace, ch, self.line, self.current),
                ':' => {
                    if self.is_next_char_match('=') {
                        self.current += 1;
                        token!(Assign, ":=", self.line, self.current)
                    } else {
                        token!(Colon, ch, self.line, self.current)
                    }
                }
                // string literal
                '"' => {
                    let string_literal = self.consume_string()?;
                    token!(String, ch, Some(LiteralValue::String(string_literal)), self.line, self.current)
                },

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_char_tokens() {
        use TokenType::*;
        let input = "# ; = : ( ) { } + - * /";
        let expected = vec![
            Fps, Semicolon, Equals, Colon, OpenParen, CloseParen, OpenBrace, CloseBrace, Plus, Minus, Star, Slash, Eof,
        ];

        let mut scanner = FpsInput::new(input);
        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 13); //Eof counts as a Token
        assert_eq!(
            scanner.tokens.into_iter().map(|x| x.token_type).collect::<Vec<TokenType>>(),
            expected
        );
    }

    #[test]
    fn two_char_tokens() {
        use TokenType::*;
        let input = ":=";
        let expected = vec![Assign, Eof];

        let mut scanner = FpsInput::new(input);
        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 2); //Eof counts as a Token
        assert_eq!(
            scanner.tokens.into_iter().map(|x| x.token_type).collect::<Vec<TokenType>>(),
            expected
        );
    }

    #[test]
    fn comment() {
        use TokenType::*;
        let input = "//I am a comment";
        let expected = vec![Comment, Eof];

        let mut scanner = FpsInput::new(input);
        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 2); //Eof counts as a Token
        assert_eq!(scanner.tokens[0].lexeme, "I am a comment");
        assert_eq!(
            scanner.tokens.into_iter().map(|x| x.token_type).collect::<Vec<TokenType>>(),
            expected
        );
    }

    #[test]
    fn string_literal() {
        use TokenType::*;
        let input = "\"I am a string literal\"";
        let expected = vec![String, Eof];

        let mut scanner = FpsInput::new(input);
        let _ = scanner.scan_tokens();

        println!("{:?}", scanner.tokens);

        assert_eq!(scanner.tokens.len(), 2); //Eof counts as a Token
        assert_eq!(scanner.tokens[0].literal, Some(LiteralValue::String("I am a string literal".to_owned())));
        assert_eq!(
            scanner.tokens.into_iter().map(|x| x.token_type).collect::<Vec<TokenType>>(),
            expected
        );
    }
}

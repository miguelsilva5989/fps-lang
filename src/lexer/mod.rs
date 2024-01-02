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

pub struct FpsInput<'a> {
    input: &'a str,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

macro_rules! token {
    ($token_type: expr, $lexeme: expr, $literal: expr, $line: expr, $pos: expr) => {
        Token::new($token_type, $lexeme, $literal, $line, $pos)
    };
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

    fn create_token(&self, token_type: TokenType, lexeme: String, literal: Option<LiteralValue>) -> Token {
        token!(token_type, lexeme, literal, self.line, self.current)
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

        self.tokens.push(self.create_token(TokenType::Eof, "".to_owned(), None));

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
                        println!("nextt  {next}");
                        if chars.contains(&next) {
                            break;
                        } else {
                            consumed.push_str(next.to_string().as_str());
                            self.current += 1;
                        }
                    } else {
                        return Err(FpsError::UnterminatedConsumption(chars, consumed, self.line).into());
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
            use TokenType::*;
            let token = match ch {
                // whitespaces
                ' ' | '\t' => self.create_token(Whitespace, ch.into(), None),
                '\n' | '\r' => {
                    self.line += 1;
                    self.create_token(Whitespace, ch.into(), None)
                }
                // operations
                '+' => self.create_token(Plus, ch.into(), None),
                '-' => self.create_token(Minus, ch.into(), None),
                '*' => self.create_token(Star, ch.into(), None),
                '/' => {
                    // comments are read until Eol
                    if self.is_next_char_match('/') {
                        self.current += 1;
                        let comment = self.read_until_eol()?;
                        self.create_token(Comment, comment.into(), None)
                    } else {
                        self.create_token(Slash, ch.into(), None)
                    }
                }
                // literals
                '#' => self.create_token(Fps, ch.into(), None),
                ';' => self.create_token(Semicolon, ch.into(), None),
                '=' => self.create_token(Equals, ch.into(), None),
                '(' => self.create_token(OpenParen, ch.into(), None),
                ')' => self.create_token(CloseParen, ch.into(), None),
                '{' => self.create_token(OpenBrace, ch.into(), None),
                '}' => self.create_token(CloseBrace, ch.into(), None),
                ':' => {
                    if self.is_next_char_match('=') {
                        self.current += 1;
                        self.create_token(Assign, ":=".to_owned(), None)
                    } else {
                        self.create_token(Colon, ch.into(), None)
                    }
                }
                // string literal
                '"' => {
                    let string_literal = self.consume_string()?;
                    self.create_token(String, string_literal.clone(), Some(LiteralValue::String(string_literal)))
                }

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
        let input = "//I am a comment\n";
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

        assert_eq!(scanner.tokens.len(), 2); //Eof counts as a Token
        assert_eq!(
            scanner.tokens[0].literal,
            Some(LiteralValue::String("I am a string literal".to_owned()))
        );
        assert_eq!(
            scanner.tokens.into_iter().map(|x| x.token_type).collect::<Vec<TokenType>>(),
            expected
        );
    }

    #[test]
    fn unterminated_consumption() {
        let input = "\"I do not end...";
        
        let mut scanner = FpsInput::new(input);
        let result = scanner.scan_tokens();

        assert_eq!(format!("{}", result.unwrap_err().root_cause()), "Unterminated consumption until char '['\"']' at line 1. Consumed: I do not end...");
    }
}

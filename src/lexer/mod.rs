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
    Eol,
    Eof,
}

#[derive(Debug, PartialEq)]
pub enum LiteralValue {
    Int(i64),
    // Float(f64),
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
            "{:?} {} {:?} line {} pos {}",
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
            line: 1,
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

        self.line += 1;
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

    fn consume_until_eol(&mut self) -> Result<String> {
        self.consume_until(vec!['\n', '\r'])
    }

    fn consume_string(&mut self) -> Result<String> {
        let consumed = self.consume_until(vec!['"']);
        self.current += 1;
        consumed
    }

    fn consume_number(&mut self) -> String {
        let mut consumed = "".to_owned();
        loop {
            match self.peek() {
                Ok(is_next) => {
                    if let Some(next) = is_next {
                        if next.is_digit(10) {
                            consumed.push_str(next.to_string().as_str());
                            self.current += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                //Eof
                Err(_) => break,
            }
        }

        consumed
    }

    fn consume_identifier(&mut self) -> String {
        let mut consumed = "".to_owned();
        loop {
            match self.peek() {
                Ok(is_next) => {
                    if let Some(next) = is_next {
                        if next.is_alphanumeric() {
                            consumed.push_str(next.to_string().as_str());
                            self.current += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                //Eof
                Err(_) => break,
            }
        }

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
                    self.create_token(Eol, ch.into(), None)
                }
                // operations
                '+' => self.create_token(Plus, ch.into(), None),
                '-' => self.create_token(Minus, ch.into(), None),
                '*' => self.create_token(Star, ch.into(), None),
                '/' => {
                    // comments are read until Eol
                    if self.is_next_char_match('/') {
                        self.current += 1;
                        let comment = self.consume_until_eol()?;
                        self.create_token(Comment, comment.into(), None)
                    } else {
                        self.create_token(Slash, ch.into(), None)
                    }
                }
                // single char
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
                // literals
                '"' => {
                    let string_literal = self.consume_string()?;
                    self.create_token(String, string_literal.clone(), Some(LiteralValue::String(string_literal)))
                }

                _ => {
                    if ch.is_digit(10) {
                        let mut num: std::string::String = ch.into();
                        num.push_str(self.consume_number().as_str());

                        self.create_token(Number, num.clone(), Some(LiteralValue::Int(num.parse::<i64>().unwrap())))
                    } else if ch.is_alphabetic() {
                        let mut id: std::string::String = ch.into();
                        id.push_str(self.consume_identifier().as_str());

                        self.create_token(Identifer, id.clone(), Some(LiteralValue::Identifier(id)))

                        // // if RESERVED.contains(&id.as_str()) {
                        // //     match id.as_str() {
                        // //         "None" => tokens.push(Token::new(id, TokenType::None)),
                        // //         _ => panic!("need to implement reserverd '{}' keywork in tokenizer", id),
                        // //     }
                        // // } else {
                        // //     tokens.push(Token::new(id, TokenType::Identifier));
                        // // }
                        // todo!()
                    } else {
                        self.current += 1;
                        return Err(FpsError::UnrecognizedChar(ch, self.line).into());
                    }
                }
            };

            return Ok(Some(token));
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenType::*;

    #[test]
    fn single_char_tokens() {
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
        let input = "//I am a comment\n";
        let expected = vec![Comment, Eol, Eof];

        let mut scanner = FpsInput::new(input);
        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 3); //Eof counts as a Token
        assert_eq!(scanner.tokens[0].lexeme, "I am a comment");
        assert_eq!(
            scanner.tokens.into_iter().map(|x| x.token_type).collect::<Vec<TokenType>>(),
            expected
        );
    }

    #[test]
    fn string_literal() {
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
    fn numeric_literal() {
        let input = "123";
        let expected = vec![Number, Eof];

        let mut scanner = FpsInput::new(input);
        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 2); //Eof counts as a Token
        assert_eq!(scanner.tokens[0].literal, Some(LiteralValue::Int(123)));
        assert_eq!(
            scanner.tokens.into_iter().map(|x| x.token_type).collect::<Vec<TokenType>>(),
            expected
        );
    }

    #[test]
    fn identifer_literal() {
        let input = "id";
        let expected = vec![Identifer, Eof];

        let mut scanner = FpsInput::new(input);
        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 2); //Eof counts as a Token
        assert_eq!(scanner.tokens[0].literal, Some(LiteralValue::Identifier("id".to_owned())));
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

        assert_eq!(
            format!("{}", result.unwrap_err().root_cause()),
            "Unterminated consumption until char '['\"']' at line 1. Consumed: I do not end..."
        );
    }
}

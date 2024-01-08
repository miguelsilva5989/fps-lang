use lazy_static::lazy_static;
use std::string::ToString;
use std::{
    collections::HashMap,
    fmt::{self, Debug, Display, Formatter},
};
use strum_macros::Display;

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
enum LexerError {
    #[error("Unrecognized char '{0}' at line {1}")]
    UnrecognizedChar(char, usize),
    #[error("Could not parse range at line {0}. Expected a digit after ..")]
    Range(usize),
    #[error("Unterminated consumption until char '{0:?}' at line {2}. Consumed: {1}")]
    UnterminatedConsumption(Vec<char>, String, usize),
}
lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        use TokenType::*;
        HashMap::from([
            ("let", Declaration),
            ("it", It), // Reference to index inside the for loop
            ("if", If),
            ("else", Else),
            ("for", For),
            ("while", While),
            ("print", Print),
            ("println", Println),
            ("true", True),
            ("false", False),
            ("null", Null),
        ])
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Display)]
pub enum TokenType {
    // single char
    Fps,    // #
    FpsEnd, // ## End program
    Semicolon,
    Colon,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    Plus,
    Minus,
    Star,
    Slash,

    // 1/2 chars long
    Equal,
    EqualEqual,
    Bang,      // !
    BangEqual, // !=
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // types
    True,
    False,
    Null,
    Range,
    RangeEqual,

    // literals
    Identifer,
    StringLiteral,
    Number,

    // keywords
    Declaration, // let
    If,
    Else,
    For,
    While,
    And,
    Or,
    It, // Reference to index inside the for loop
    Print,
    Println,

    // Ignore
    Comment,
    Whitespace,
    Eol,
    Eof,
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum LiteralValue {
    Float(f64),
    StringValue(String),
    Identifier(String),
    Keyword(String),
    Fps(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
    pub line: usize,
    pub pos: usize,
    // pub fps: usize,
}

impl Display for Token {
    fn fmt(&self, format: &mut Formatter) -> fmt::Result {
        match &self.literal {
            Some(literal) => write!(
                format,
                "{} {} {} line {} pos {}",
                self.token_type, self.lexeme, literal, self.line, self.pos
            ),
            None => write!(
                format,
                "{} {} None line {} pos {}",
                self.token_type, self.lexeme, self.line, self.pos
            ),
        }
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
            // fps,
        }
    }
}

pub struct FpsInput<'a> {
    input: &'a str,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    // current_fps: usize,
}

impl Display for FpsInput<'_> {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        write!(format, "Lexer:\n")?;
        for v in &self.tokens {
            write!(format, "\t{}\n", v)?;
        }
        Ok(())
    }
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
            // current_fps: 0,
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
                match token.token_type {
                    // ignore whitespaces
                    TokenType::Whitespace => {}
                    TokenType::Eol => {
                        self.line += 1;
                    }
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

    fn advance(&mut self) -> Result<Option<char>> {
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
                        return Err(LexerError::UnterminatedConsumption(chars, consumed, self.line).into());
                    }
                }
                //Eof
                Err(_) => return Err(LexerError::UnterminatedConsumption(chars, consumed, self.line).into()),
            }
        }
        Ok(consumed)
    }

    fn next_chars_match(&mut self, chars: Vec<char>) -> Result<bool> {
        let current = self.current;
        let mut is_same = false;
        for ch in chars {
            if self.is_next_char_match(ch) {
                is_same = true;
                self.advance()?;
            } else {
                is_same = false;
                break;
            }
        }

        // go back to initial position if chars don't match
        if !is_same {
            self.current = current;
        }

        Ok(is_same)
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

    fn consume_fps(&mut self) -> Result<String> {
        let mut consumed = "#".to_owned();

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
                    }
                }
                Err(_) => (),
            }
        }

        Ok(consumed)
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
                        } else if self.is_next_char_match('.') {
                            // check if consumed already has more dots??
                            if consumed.contains(".") {
                                break;
                            }

                            consumed.push_str(next.to_string().as_str());
                            self.current += 1;
                            consumed += self.consume_number().as_str();
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

    fn consume_range(&mut self) -> Result<String> {
        let mut consumed = "..".to_owned();

        if self.is_next_char_match('=') {
            consumed.push(self.advance().unwrap().unwrap());
        }

        match self.peek() {
            Ok(is_next) => {
                if let Some(next) = is_next {
                    if next.is_digit(10) {
                        consumed.push_str(next.to_string().as_str());
                        self.current += 1;
                    } else {
                        return Err(LexerError::Range(self.line).into());
                    }
                } else {
                    return Err(LexerError::Range(self.line).into());
                }
            }
            //Eof
            Err(_) => return Err(LexerError::Range(self.line).into()),
        }

        Ok(consumed)
    }

    fn tokenzine(&mut self) -> Result<Option<Token>> {
        if let Ok(Some(ch)) = self.advance() {
            use TokenType::*;
            let token = match ch {
                // whitespaces
                ' ' | '\t' | '\r' => self.create_token(Whitespace, "WS".to_owned(), None),
                '\n' => self.create_token(Eol, "Eol".to_owned(), None),
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
                ':' => self.create_token(Colon, ch.into(), None),
                ';' => self.create_token(Semicolon, ch.into(), None),
                '(' => self.create_token(OpenParen, ch.into(), None),
                ')' => self.create_token(CloseParen, ch.into(), None),
                '{' => self.create_token(OpenBrace, ch.into(), None),
                '}' => self.create_token(CloseBrace, ch.into(), None),
                // single or double char
                '#' => {
                    // self.current_fps += 1;
                    if self.is_next_char_match('#') {
                        self.current += 1;
                        self.create_token(FpsEnd, "##".to_owned(), None)
                    } else {
                        let fps = self.consume_fps()?;
                        if fps.len() > 1 {
                            let fps_count = fps.replace("#", "").parse::<usize>().unwrap();
                            self.create_token(Fps, format!("#{}", fps_count), Some(LiteralValue::Fps(fps_count)))
                        } else {
                            self.create_token(Fps, ch.into(), Some(LiteralValue::Fps(1)))
                        }
                    }
                }
                '=' => {
                    if self.is_next_char_match('=') {
                        self.current += 1;
                        self.create_token(EqualEqual, "==".to_owned(), None)
                    } else {
                        self.create_token(Equal, ch.into(), None)
                    }
                }
                '!' => {
                    if self.is_next_char_match('=') {
                        self.current += 1;
                        self.create_token(BangEqual, "!=".to_owned(), None)
                    } else {
                        self.create_token(Bang, ch.into(), None)
                    }
                }
                '>' => {
                    if self.is_next_char_match('=') {
                        self.current += 1;
                        self.create_token(GreaterEqual, ">=".to_owned(), None)
                    } else {
                        self.create_token(Greater, ch.into(), None)
                    }
                }
                '<' => {
                    if self.is_next_char_match('=') {
                        self.current += 1;
                        self.create_token(LessEqual, "<=".to_owned(), None)
                    } else {
                        self.create_token(Less, ch.into(), None)
                    }
                }
                '&' => {
                    if self.is_next_char_match('&') {
                        self.current += 1;
                        self.create_token(And, "&&".to_owned(), None)
                    } else {
                        self.match_default(ch)?
                    }
                }
                '|' => {
                    if self.is_next_char_match('|') {
                        self.current += 1;
                        self.create_token(Or, "||".to_owned(), None)
                    } else {
                        self.match_default(ch)?
                    }
                }
                // literals
                '"' => {
                    let string_literal = self.consume_string()?;
                    self.create_token(
                        StringLiteral,
                        string_literal.clone(),
                        Some(LiteralValue::StringValue(string_literal)),
                    )
                }

                _ => {
                    self.match_default(ch)?
                }
            };

            return Ok(Some(token));
        }
        Ok(None)
    }

    fn match_default(&mut self, ch: char) -> Result<Token> {
        use TokenType::*;
        let token = if ch.is_digit(10) {
            let mut num: String = ch.into();
            if self.next_chars_match(vec!['.', '.'])? {
                let mut range = num;
                range.push_str(self.consume_range()?.as_str());
                if range.contains("=") {
                    self.create_token(RangeEqual, range.clone(), None)
                } else {
                    self.create_token(Range, range.clone(), None)
                }
            } else {
                num.push_str(self.consume_number().as_str());
                self.create_token(Number, num.clone(), Some(LiteralValue::Float(num.parse::<f64>().unwrap())))
            }
        } else if ch.is_alphabetic() {
            let mut id: String = ch.into();
            id.push_str(self.consume_identifier().as_str());

            if let Some(tt) = KEYWORDS.get(id.as_str()) {
                self.create_token(*tt, id.clone(), Some(LiteralValue::Keyword(id)))
            } else {
                self.create_token(Identifer, id.clone(), Some(LiteralValue::Identifier(id)))
            }
        } else {
            self.current += 1;
            return Err(LexerError::UnrecognizedChar(ch, self.line).into());
        };

        return Ok(token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use TokenType::*;

    #[test]
    fn single_char_tokens() {
        let input = "# ; = : ( ) { } + - * / ! > <";
        let expected = vec![
            Fps, Semicolon, Equal, Colon, OpenParen, CloseParen, OpenBrace, CloseBrace, Plus, Minus, Star, Slash, Bang, Greater, Less, Eof,
        ];

        let mut scanner = FpsInput::new(input);
        let _tokens = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 16); //Eof counts as a Token
        assert_eq!(
            scanner.tokens.into_iter().map(|x| x.token_type).collect::<Vec<TokenType>>(),
            expected
        );
    }

    #[test]
    fn two_char_tokens() {
        let input = "#10 == != >= <= && || ";
        let expected = vec![Fps, EqualEqual, BangEqual, GreaterEqual, LessEqual, And, Or, Eof];

        let mut scanner = FpsInput::new(input);
        let _tokens = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 8); //Eof counts as a Token
        assert_eq!(
            scanner.tokens.into_iter().map(|x| x.token_type).collect::<Vec<TokenType>>(),
            expected
        );
    }

    #[test]
    fn types() {
        let input = "0..1 1..=2";
        let expected = vec![Range, RangeEqual, Eof];

        let mut scanner = FpsInput::new(input);
        let _tokens = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 3); //Eof counts as a Token
        assert_eq!(
            scanner.tokens.into_iter().map(|x| x.token_type).collect::<Vec<TokenType>>(),
            expected
        );
    }

    #[test]
    fn comment() {
        let input = "//I am a comment\n";
        let expected = vec![Comment, Eof];

        let mut scanner = FpsInput::new(input);
        let _tokens = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 2); //Eof counts as a Token
        assert_eq!(scanner.tokens[0].lexeme, "I am a comment");
        assert_eq!(
            scanner.tokens.into_iter().map(|x| x.token_type).collect::<Vec<TokenType>>(),
            expected
        );
    }

    #[test]
    fn string_literal() {
        let input = "\"I am a string literal\"";
        let expected = vec![StringLiteral, Eof];

        let mut scanner = FpsInput::new(input);
        let _tokens = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 2); //Eof counts as a Token
        assert_eq!(
            scanner.tokens[0].literal,
            Some(LiteralValue::StringValue("I am a string literal".to_owned()))
        );
        assert_eq!(
            scanner.tokens.into_iter().map(|x| x.token_type).collect::<Vec<TokenType>>(),
            expected
        );
    }

    #[test]
    fn numeric_literal() {
        let input = "123.123";
        let expected = vec![Number, Eof];

        let mut scanner = FpsInput::new(input);
        let _tokens = scanner.scan_tokens();

        // println!("{:?}", _tokens);

        assert_eq!(scanner.tokens.len(), 2); //Eof counts as a Token
        assert_eq!(scanner.tokens[0].literal, Some(LiteralValue::Float(123.123)));
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
        let _tokens = scanner.scan_tokens();

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

    #[test]
    fn keywords() {
        let input = "for forca print println";
        let expected = vec![For, Identifer, Print, Println, Eof];

        let mut scanner = FpsInput::new(input);
        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 5); //Eof counts as a Token
        assert_eq!(scanner.tokens[0].literal, Some(LiteralValue::Keyword("for".to_owned())));
        assert_eq!(scanner.tokens[2].literal, Some(LiteralValue::Keyword("print".to_owned())));
        assert_eq!(scanner.tokens[3].literal, Some(LiteralValue::Keyword("println".to_owned())));
        assert_eq!(
            scanner.tokens.into_iter().map(|x| x.token_type).collect::<Vec<TokenType>>(),
            expected
        );
    }
}

use std::io::{Error, ErrorKind};

use crate::token::{Token, TokenType};

pub struct Lexer {
    source: Vec<char>,
    current_char: char,
    current_index: usize,
    current_line: usize,
    current_column: usize
}

impl Lexer {
    pub fn new(source_code: String) -> Self {
        let source: Vec<char> = source_code.chars().collect();
        let first_char: char = source[0].clone();
        Self {
            source,
            current_char: first_char,
            current_index: 0,
            current_line: 1,
            current_column: 0
        }
    }

    fn is_end_of_file(&self) -> bool {
        self.current_char == '\0' || self.current_index >= self.source.len()
    }

    fn advance(&mut self) {
        if !self.is_end_of_file() {
            if let Some(next_char) = self.source.get(self.current_index + 1) {
                if self.current_char == '\n' {
                    self.current_line += 1;
                    self.current_column = 0;
                } else {
                    self.current_column += 1;
                }
                self.current_char = *next_char;
                self.current_index += 1;
                return
            }
        }
        self.current_char = '\0';
    }

    fn peek(&self, offset: usize) -> char {
        if !self.is_end_of_file() {
            if let Some(ch) = self.source.get(self.current_index + offset) {
                return *ch;
            }
        }
        '\0'
    }

    fn skip_whitespace(&mut self) {
        while self.current_char.is_whitespace() && !self.is_end_of_file() {
            self.advance();
        }
    }

    fn skip_line(&mut self) {
        while self.current_char != '\n' && !self.is_end_of_file() {
            self.advance();
        }
    }

    fn token_char_advance(&mut self, token_type: TokenType) -> Token {
        let token: Token = Token::new(self.current_char.to_string(), token_type, self.current_line, self.current_column);
        self.advance();
        token
    }

    fn scan_number(&mut self) -> Token {
        let mut value: String = self.current_char.to_string();
        let column: usize = self.current_column;
        self.advance();
        while self.current_char.is_numeric() {
            value.push(self.current_char);
            self.advance();
        }
        Token::new(value, TokenType::Number, self.current_line, column)
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, Error> {
        let mut result: Vec<Token> = Vec::new();
        while !self.is_end_of_file() {
            self.skip_whitespace();
            match self.current_char {
                '<' => result.push(self.token_char_advance(TokenType::LessThan)),
                '>' => result.push(self.token_char_advance(TokenType::GreaterThan)),
                '[' => result.push(self.token_char_advance(TokenType::LeftParen)),
                ']' => result.push(self.token_char_advance(TokenType::RightParen)),
                '@' => result.push(self.token_char_advance(TokenType::Arobase)),
                '%' => self.skip_line(),
                ch => {
                    if ch.is_alphabetic() {
                        let mut value: String = ch.to_string();
                        let column: usize = self.current_column;
                        self.advance();
                        if self.current_char == '+' || self.current_char == '#' {
                            value.push(self.current_char);
                            self.advance();
                        }
                        let token: Token = Token::new(value, TokenType::Command, self.current_line, column);
                        result.push(token);
                    } else if ch.is_numeric() {
                        result.push(self.scan_number());
                    } else if !self.is_end_of_file() {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            format!("Unexpected character \"{}\" at line {}, column {}.", ch, self.current_line, self.current_column)
                        ))
                    }
                }
            }
        }
        result.push(Token::empty(self.current_line, self.current_column));
        Ok(result)
    }
}

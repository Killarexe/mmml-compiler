use std::io::{Error, ErrorKind};

use crate::token::{Token, TokenType};

const DURATION_VALUES: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];

const BASIC_COMMANDS: [&'static str; 13] = [
    "R", "C", "C#", "D", "D#", "E",
    "F", "F#", "G", "G#", "A", "A#", "B"
];

pub struct Compiler {
    tokens: Vec<Token>,
    current_token: Token,
    current_index: usize,
    current_octave: u8,
    num_of_headers: u8
}

impl Compiler {
    pub fn new(tokens: Vec<Token>) -> Self {
        let first_token: Token = tokens[0].clone();
        Self {
            tokens,
            current_token: first_token,
            current_index: 0,
            current_octave: 1,
            num_of_headers: 0
        }
    }

    fn is_end_of_file(&self) -> bool {
        self.current_token.token_type == TokenType::EndOfFile || self.current_index >= self.tokens.len()
    }

    fn advance(&mut self) {
        if !self.is_end_of_file() {
            if let Some(token) = self.tokens.get(self.current_index + 1) {
                self.current_index += 1;
                self.current_token = token.clone();
                return;
            }
        }
        self.current_token = Token::empty(self.current_token.line, self.current_token.column);
    }

    fn compile_number(&mut self) -> Result<u8, Error> {
        if self.current_token.token_type != TokenType::Number {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Tried to convert a non number token '{}' at line {}, column {}.", self.current_token.value, self.current_token.line, self.current_token.column)
            ));
        }
        if let Ok(number) = u8::from_str_radix(&self.current_token.value, 10) {
            self.advance();
            return Ok(number);
        }

        Err(Error::new(
            ErrorKind::InvalidData,
            format!("Failed to convert number '{}' at line {}, column {}.", self.current_token.value, self.current_token.line, self.current_token.column)
        ))
    }

    fn compile_duration_number(&mut self) -> Result<u8, Error> {
        let number_token: Token = self.current_token.clone();
        let number: u8 = self.compile_number()?;
        let is_dotted: bool = self.current_token.token_type == TokenType::Dot;
        if let Some(duration_number) = DURATION_VALUES.iter().position(|&x| x == number) {
            if is_dotted {
                self.advance();
                return Ok(0x8 | duration_number as u8);
            }
            return Ok(duration_number as u8);
        }
        Err(Error::new(
            ErrorKind::InvalidData,
            format!("Invalid duration number '{}' at line {}, column {}.\nExpected 128, 64, 64., 32, 32., 16, 16., 8, 8., 4, 4., 2, 2. or 1", number, number_token.line, number_token.column)
        ))
    }

    fn compile_command(&mut self) -> Result<Vec<u8>, Error> {
        let command_token: Token = self.current_token.clone();
        let command_name: String = command_token.value.to_uppercase();
        let command_name: &str = command_name.as_str();
        self.advance();
        if let Some(command) = BASIC_COMMANDS.iter().position(|&x| x == command_name) {
            let duration_number: u8 = self.compile_duration_number()?;
            return Ok(vec![((command as u8) << 4) | duration_number]);
        }
        let number: u8 = self.compile_number()?;
        match command_name {
            "O" => {
                if number > 0 && number < 6 {
                    self.current_octave = number;
                    return Ok(vec![0xD0 | number]);
                }
                Err(Error::new(
                    ErrorKind::Unsupported,
                    format!("Invalid octave number at line {}, column {}:\nExpected octave number 1-5.", self.current_token.line, self.current_token.column)
                ))
            },
            "V" => {
                if number < 9 {
                    return Ok(vec![0xE0 | 9 - number]);
                }
                Err(Error::new(
                    ErrorKind::Unsupported,
                    format!("Invalid volume number at line {}, column {}:\nExpected volume number 0-8.", self.current_token.line, self.current_token.column)
                ))
            },
            "T" => {
                Ok(vec![0xF3, number])
            },
            "M" => {
                let num_of_macros: u8 = self.num_of_headers - 3;
                if number <= num_of_macros {
                    return Ok(vec![0xF2, number]);
                }
                Err(Error::new(
                    ErrorKind::Unsupported,
                    format!("Invalid macro number at line {}, column {}:\nNumber of macros: {}.", self.current_token.line, self.current_token.column, num_of_macros)
                ))
            },
            _ => {
                Err(Error::new(
                    ErrorKind::Unsupported,
                    format!("Unexpected command called {} at line {} column {}.", command_name, command_token.line, command_token.column)
                ))
            }
        }
    }

    fn compile_token(&mut self) -> Result<Vec<u8>, Error> {
        match self.current_token.token_type {
            TokenType::Arobase => {
                self.advance();
                Ok(vec![0xFF])
            },
            TokenType::LessThan => {
                if self.current_octave <= 1 {
                    return Err(Error::new(
                        ErrorKind::Unsupported,
                        format!("Octave error at line {}, column {}:\nTried to lower octave by 1 but the octave was already at is minimum.", self.current_token.line, self.current_token.column)
                    ));
                }
                self.current_octave -= 1;
                self.advance();
                Ok(vec![0xD0 | self.current_octave])
            },
            TokenType::GreaterThan => {
                if self.current_octave >= 5 {
                    return Err(Error::new(
                        ErrorKind::Unsupported,
                        format!("Octave error at line {}, column {}:\nTried to upper octave by 1 but the octave was already at is maximum.", self.current_token.line, self.current_token.column)
                    ));
                }
                self.current_octave += 1;
                self.advance();
                Ok(vec![0xD0 | self.current_octave])
            }
            TokenType::LeftParen => {
                self.advance();
                let times: u8 = self.compile_number()?;
                Ok(vec![0xF0, times])
            }
            TokenType::RightParen => {
                self.advance();
                Ok(vec![0xF1])
            }
            TokenType::Command => self.compile_command(),
            TokenType::EndOfFile => {
                Err(Error::new(
                    ErrorKind::UnexpectedEof,
                    format!("Unexpected end of file {} at line {}, column {}.\nExpected @, <, >, [, ] or a command.", self.current_token.value, self.current_token.line, self.current_token.column)
                ))
            }
            _ => {
                Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Unexpected token {} at line {}, column {}.\nExpected @, <, >, [, ] or a command.", self.current_token.value, self.current_token.line, self.current_token.column)
                ))
            }
        }
    }

    pub fn compile(&mut self) -> Result<Vec<u8>, Error> {
        let mut result: Vec<u8> = Vec::new();
        let num_of_headers: usize = self.tokens.clone().into_iter()
            .filter(|token| token.token_type == TokenType::Arobase)
            .collect::<Vec<Token>>()
            .len() - 1;
        if num_of_headers > 254 {
            return Err(Error::new(
                ErrorKind::Unsupported,
                format!("MMML files can support 255 headers max. Found {}", num_of_headers)
            ));
        } else if num_of_headers < 4 {
            return Err(Error::new(
                ErrorKind::Unsupported,
                format!("MMML files require 4 headers min. Found {}", num_of_headers)
            ));
        }

        result.append(&mut vec![0; num_of_headers * 2]);

        if self.current_token.token_type != TokenType::Arobase {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "The file do not start with an '@'.".to_string()
            ));
        }
        self.advance();

        let mut headers_positions: Vec<usize> = Vec::new();

        while !self.is_end_of_file() {
            let mut compiled_command: Vec<u8> = self.compile_token()?;
            if compiled_command == [0xFF] {
                headers_positions.push(result.len());
            }
            result.append(&mut compiled_command);
        }

        if result.len() > u16::MAX.into() {
            return Err(Error::new(
                ErrorKind::Unsupported,
                format!("Compiled music program if over the 16-bit limit!\nProgram size: {}", result.len())
            ));
        }

        for (index, &position) in headers_positions.iter().enumerate() {
            let header_pos: u16 = position as u16;
            result[index * 2] = (header_pos >> 8) as u8;
            result[index * 2 + 1] = (header_pos & 0xFF) as u8;
        }
        Ok(result)
    }
}

use std::io::{Error, ErrorKind};

use crate::{expression::Expression, statement::Statement, token::{Token, TokenType}};

const COMMANDS: [&'static str; 17] = [
    "R", "C", "C#", "D", "D#", "E", "F", "F#",
    "G", "G#", "A", "A#", "B", "O", "M", "T",
    "V"
];

pub struct Parser {
    tokens: Vec<Token>,
    current_token: Token,
    current_index: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let current_token: Token = tokens[0].clone();
        Self {
            tokens,
            current_token,
            current_index: 0
        }
    }

    fn is_end_of_file(&self) -> bool {
        self.current_index >= self.tokens.len() || self.current_token.is_end_of_file()
    }

    fn eat(&mut self) -> Token {
        let previous_token: Token = self.current_token.clone();
        if !self.is_end_of_file() {
            if let Some(token) = self.tokens.get(self.current_index + 1) {
                self.current_index += 1;
                self.current_token = token.clone();
                return previous_token;
            }
        }
        self.current_token = Token::empty(self.current_token.line, self.current_token.column);
        previous_token
    }

    fn check(&self, token_type: TokenType) -> bool {
        !self.is_end_of_file() && self.current_token.token_type == token_type
    }

    fn match_eat(&mut self, token_type: TokenType) -> Result<Token, Error> {
        if self.check(token_type) {
            return Ok(self.eat());
        }
        Err(Error::new(
            ErrorKind::NotFound,
            format!(
                "Unexpected token type {:#?}, expected {:#?}. At line {}, column {}.",
                self.current_token.token_type, token_type, self.current_token.line, self.current_token.column
            )
        ))
    }

    fn parse_number(&mut self) -> Result<u8, Error> {
        let number: Token = self.match_eat(TokenType::Number)?;
        if let Ok(number) = u8::from_str_radix(number.value.as_str(), 10) {
            return Ok(number);
        }
        Err(Error::new(
            ErrorKind::InvalidData,
            format!("Can't format the number {}. At line {}, column {}.", number.value, number.line, number.column)
        ))
    }

    fn parse_command(&mut self) -> Result<Statement, Error> {
        let command: Token = self.match_eat(TokenType::Command)?;
        let command_str: String = command.value.to_uppercase().replace('+', "#");
        let argument: u8 = if self.current_token.token_type == TokenType::Number {
            self.parse_number()?
        } else {
            0
        };

        if COMMANDS.contains(&command_str.as_str()) {
            return Ok(Statement::ExpressionStatement {
                expression: Expression::Command {
                    command,
                    argument
                }
            });
        }

        Err(Error::new(
            ErrorKind::NotFound,
            format!(
                "No commands named '{}'. At line {}, column {}.",
                command.value, command.line, command.column
            )
        ))
    }

    fn parse_loop(&mut self) -> Result<Statement, Error> {
        let mut result: Vec<Statement> = Vec::new();
        self.match_eat(TokenType::LeftParen)?;
        let times: u8 = self.parse_number()?;

        while !self.is_end_of_file() && self.current_token.token_type != TokenType::RightParen {
            let statement: Statement = self.parse_statement()?;
            result.push(statement);
        }

        self.match_eat(TokenType::RightParen)?;

        Ok(Statement::Loop {
            times,
            statements: result
        })
    }

    fn parse_statement(&mut self) -> Result<Statement, Error> {
        match self.current_token.token_type {
            TokenType::Command => self.parse_command(),
            TokenType::LeftParen => self.parse_loop(),
            TokenType::LessThan | TokenType::GreaterThan => Ok(Statement::ExpressionStatement {
                expression: Expression::ShortCommand {
                    command: self.eat()
                }
            }),
            _ => {
                Err(Error::new(
                    ErrorKind::NotFound,
                    format!(
                        "Unexpected token type {:#?}, expected a command or an '@'. At line {}, column {}.",
                        self.current_token.token_type, self.current_token.line, self.current_token.column
                    )
                ))
            }
        }
    }

    fn parse_block(&mut self) -> Result<Statement, Error> {
        let mut result: Vec<Statement> = Vec::new();
        self.match_eat(TokenType::Arobase)?;
        while !self.is_end_of_file() && self.current_token.token_type != TokenType::Arobase {
            let statement: Statement = self.parse_statement()?;
            result.push(statement);
        }
        Ok(Statement::Program { body: result })
    }

    pub fn parse(&mut self) -> Result<Statement, Error> {
        let mut result: Vec<Statement> = Vec::new();
        while !self.is_end_of_file() {
            let statement: Statement = self.parse_block()?;
            result.push(statement);
        }
        Ok(Statement::Program { body: result })
    }
}

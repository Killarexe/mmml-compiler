#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Command,
    LessThan,
    GreaterThan,
    LeftParen,
    RightParen,
    Arobase,
    Number,
    EndOfFile
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize
}

impl Token {
    pub fn new(value: String, token_type: TokenType, line: usize, column: usize) -> Self {
        Self{
            value,
            token_type,
            line,
            column
        } 
    }

    pub fn empty(line: usize, column: usize) -> Self {
        Self {
            value: String::new(),
            token_type: TokenType::EndOfFile,
            line,
            column
        }
    }

    pub fn is_end_of_file(&self) -> bool {
        self.token_type == TokenType::EndOfFile
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("Token \"{}\" of type {:#?} at line {}, column {}", self.value, self.token_type, self.line, self.column)
    }
}

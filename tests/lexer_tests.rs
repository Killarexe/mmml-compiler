use mmml_compiler::{lexer::Lexer, token::Token, token::TokenType};

#[test]
fn test_commands() {
    let source: String = "c1c#2d4d#8e16f32f#64g128g#1a2a#4b8r16&".into();
    let mut lexer: Lexer = Lexer::new(source);
    let tokens: Vec<Token> = lexer.tokenize().unwrap();
    let expected_tokens: Vec<Token> = vec![
        Token::new("c".into(), TokenType::Command, 1, 0),
        Token::new("1".into(), TokenType::Number, 1, 1),
        Token::new("c#".into(), TokenType::Command, 1, 2),
        Token::new("2".into(), TokenType::Number, 1, 4),
        Token::new("d".into(), TokenType::Command, 1, 5),
        Token::new("4".into(), TokenType::Number, 1, 6),
        Token::new("d#".into(), TokenType::Command, 1, 7),
        Token::new("8".into(), TokenType::Number, 1, 9),
        Token::new("e".into(), TokenType::Command, 1, 10),
        Token::new("16".into(), TokenType::Number, 1, 11),
        Token::new("f".into(), TokenType::Command, 1, 13),
        Token::new("32".into(), TokenType::Number, 1, 14),
        Token::new("f#".into(), TokenType::Command, 1, 16),
        Token::new("64".into(), TokenType::Number, 1, 18),
        Token::new("g".into(), TokenType::Command, 1, 20),
        Token::new("128".into(), TokenType::Number, 1, 21),
        Token::new("g#".into(), TokenType::Command, 1, 24),
        Token::new("1".into(), TokenType::Number, 1, 26),
        Token::new("a".into(), TokenType::Command, 1, 27),
        Token::new("2".into(), TokenType::Number, 1, 28),
        Token::new("a#".into(), TokenType::Command, 1, 29),
        Token::new("4".into(), TokenType::Number, 1, 31),
        Token::new("b".into(), TokenType::Command, 1, 32),
        Token::new("8".into(), TokenType::Number, 1, 33),
        Token::new("r".into(), TokenType::Command, 1, 34),
        Token::new("16".into(), TokenType::Number, 1, 35),
        Token::new("&".into(), TokenType::Command, 1, 37),
        Token::empty(1, 37)
    ];
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_comment() {
    let source: String = "%This is a test comment!\nc1c#2d4d#8e16f32f#64g128g#1a2a#4b8\n%And an other one!\nr16&".into();
    let mut lexer: Lexer = Lexer::new(source);
    let tokens: Vec<Token> = lexer.tokenize().unwrap();
    let expected_tokens: Vec<Token> = vec![
        Token::new("c".into(), TokenType::Command, 2, 0),
        Token::new("1".into(), TokenType::Number, 2, 1),
        Token::new("c#".into(), TokenType::Command, 2, 2),
        Token::new("2".into(), TokenType::Number, 2, 4),
        Token::new("d".into(), TokenType::Command, 2, 5),
        Token::new("4".into(), TokenType::Number, 2, 6),
        Token::new("d#".into(), TokenType::Command, 2, 7),
        Token::new("8".into(), TokenType::Number, 2, 9),
        Token::new("e".into(), TokenType::Command, 2, 10),
        Token::new("16".into(), TokenType::Number, 2, 11),
        Token::new("f".into(), TokenType::Command, 2, 13),
        Token::new("32".into(), TokenType::Number, 2, 14),
        Token::new("f#".into(), TokenType::Command, 2, 16),
        Token::new("64".into(), TokenType::Number, 2, 18),
        Token::new("g".into(), TokenType::Command, 2, 20),
        Token::new("128".into(), TokenType::Number, 2, 21),
        Token::new("g#".into(), TokenType::Command, 2, 24),
        Token::new("1".into(), TokenType::Number, 2, 26),
        Token::new("a".into(), TokenType::Command, 2, 27),
        Token::new("2".into(), TokenType::Number, 2, 28),
        Token::new("a#".into(), TokenType::Command, 2, 29),
        Token::new("4".into(), TokenType::Number, 2, 31),
        Token::new("b".into(), TokenType::Command, 2, 32),
        Token::new("8".into(), TokenType::Number, 2, 33),
        Token::new("r".into(), TokenType::Command, 4, 0),
        Token::new("16".into(), TokenType::Number, 4, 1),
        Token::new("&".into(), TokenType::Command, 4, 3),
        Token::empty(4, 3)
    ];
    assert_eq!(tokens, expected_tokens);
}

#[test]
fn test_symbols() {
    let source: String = "@[2]<>r2.".into();
    let mut lexer: Lexer = Lexer::new(source);
    let tokens: Vec<Token> = lexer.tokenize().unwrap();
    let expected_tokens: Vec<Token> = vec![
        Token::new("@".into(), TokenType::Arobase, 1, 0),
        Token::new("[".into(), TokenType::LeftParen, 1, 1),
        Token::new("2".into(), TokenType::Number, 1, 2),
        Token::new("]".into(), TokenType::RightParen, 1, 3),
        Token::new("<".into(), TokenType::LessThan, 1, 4),
        Token::new(">".into(), TokenType::GreaterThan, 1, 5),
        Token::new("r".into(), TokenType::Command, 1, 6),
        Token::new("2".into(), TokenType::Number, 1, 7),
        Token::new(".".into(), TokenType::Dot, 1, 8),
        Token::empty(1, 8)
    ];
    assert_eq!(tokens, expected_tokens);
}

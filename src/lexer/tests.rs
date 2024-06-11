use super::*;

#[test]
fn can_scan_positive_numbers() {
    let source = "0 0.5 1 2.5 3.45678";
    let lexer = Lexer::new(source);

    let tokens = lexer.scan_all_tokens();
    let expected = [
        Ok(Token {
            token_type: TokenType::Number(0.0),
            line: 1,
            col: 1,
        }),
        Ok(Token {
            token_type: TokenType::Number(0.5),
            line: 1,
            col: 5,
        }),
        Ok(Token {
            token_type: TokenType::Number(1.0),
            line: 1,
            col: 7,
        }),
        Ok(Token {
            token_type: TokenType::Number(2.5),
            line: 1,
            col: 11,
        }),
        Ok(Token {
            token_type: TokenType::Number(3.45678),
            line: 1,
            col: 19,
        }),
    ];

    for (idx, (token, expected_token)) in tokens.into_iter().zip(expected).enumerate() {
        assert_eq!(token, expected_token, "index: {idx}");
    }
}
#[test]
fn can_scan_negative_numbers() {
    let source = "-0.5 -1 -2.5 -3.45678";
    let lexer = Lexer::new(source);

    let tokens = lexer.scan_all_tokens();
    let expected = [
        Ok(Token {
            token_type: TokenType::Minus,
            line: 1,
            col: 1,
        }),
        Ok(Token {
            token_type: TokenType::Number(0.5),
            line: 1,
            col: 4,
        }),
        Ok(Token {
            token_type: TokenType::Minus,
            line: 1,
            col: 6,
        }),
        Ok(Token {
            token_type: TokenType::Number(1.0),
            line: 1,
            col: 7,
        }),
        Ok(Token {
            token_type: TokenType::Minus,
            line: 1,
            col: 9,
        }),
        Ok(Token {
            token_type: TokenType::Number(2.5),
            line: 1,
            col: 12,
        }),
        Ok(Token {
            token_type: TokenType::Minus,
            line: 1,
            col: 14,
        }),
        Ok(Token {
            token_type: TokenType::Number(3.45678),
            line: 1,
            col: 21,
        }),
    ];

    for (idx, (token, expected_token)) in tokens.into_iter().zip(expected).enumerate() {
        assert_eq!(token, expected_token, "index: {idx}");
    }
}

#[test]
fn can_scan_int() {
    let source = "3";
    let lexer = Lexer::new(source);

    let tokens = lexer.scan_all_tokens();
    let expected = [Ok(Token {
        token_type: TokenType::Number(3.0f64),
        line: 1,
        col: source.len(),
    })];

    assert_eq!(tokens, expected);
}

#[test]
fn can_scan_string_literal() {
    let source = "\"Hello, world!\"";
    let lexer = Lexer::new(source);

    let tokens = lexer.scan_all_tokens();
    let expected = [Ok(Token {
        token_type: TokenType::String("Hello, world!".to_string()),
        line: 1,
        col: source.len(),
    })];

    assert_eq!(tokens, expected);
}

#[test]
fn can_scan_booleans() {
    let source = "true false";
    let lexer = Lexer::new(source);

    let tokens = lexer.scan_all_tokens();
    let expected = [
        Ok(Token {
            token_type: TokenType::True,
            line: 1,
            col: 4,
        }),
        Ok(Token {
            token_type: TokenType::False,
            line: 1,
            col: 10,
        }),
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn can_scan_nil() {
    let source = "nil";
    let lexer = Lexer::new(source);

    let tokens = lexer.scan_all_tokens();
    let expected = [Ok(Token {
        token_type: TokenType::Nil,
        line: 1,
        col: 3,
    })];

    assert_eq!(tokens, expected);
}

#[test]
fn can_scan_rust_use_statement() {
    let source = "use anyhow::{Context, Result};";
    let lexer = Lexer::new(source);

    let tokens = lexer.scan_all_tokens();
    let expected = [
        Ok(Token {
            token_type: TokenType::Identifier("use".to_owned()),
            line: 1,
            col: 3,
        }),
        Ok(Token {
            token_type: TokenType::Identifier("anyhow".to_owned()),
            line: 1,
            col: 10,
        }),
        Err(LexerError::UnexpectedCharacter {
            character: ':',
            line: 1,
            col: 11,
        }),
        Err(LexerError::UnexpectedCharacter {
            character: ':',
            line: 1,
            col: 12,
        }),
        Ok(Token {
            token_type: TokenType::LeftBrace,
            line: 1,
            col: 13,
        }),
        Ok(Token {
            token_type: TokenType::Identifier("Context".to_owned()),
            line: 1,
            col: 20,
        }),
        Ok(Token {
            token_type: TokenType::Comma,
            line: 1,
            col: 21,
        }),
        Ok(Token {
            token_type: TokenType::Identifier("Result".to_owned()),
            line: 1,
            col: 28,
        }),
        Ok(Token {
            token_type: TokenType::RightBrace,
            line: 1,
            col: 29,
        }),
        Ok(Token {
            token_type: TokenType::Semicolon,
            line: 1,
            col: 30,
        }),
    ];

    for (idx, (token, expected_token)) in tokens.into_iter().zip(expected).enumerate() {
        assert_eq!(token, expected_token, "index: {idx}");
    }
}

#[test]
fn error_on_non_terminated_string() {
    let source = "\"Hello, world!";
    let lexer = Lexer::new(source);

    let tokens = lexer.scan_all_tokens();
    let expected = [Err(LexerError::UnterminatedString { line: 1, col: 14 })];

    assert_eq!(tokens, expected);
}

#[test]
fn error_on_common_unexpected_characters() {
    let source = "@ # $ % ^ & | \\ : ' ?";
    let lexer = Lexer::new(source);

    let tokens = lexer.scan_all_tokens();
    let expected = [
        Err(LexerError::UnexpectedCharacter {
            character: '@',
            line: 1,
            col: 1,
        }),
        Err(LexerError::UnexpectedCharacter {
            character: '#',
            line: 1,
            col: 3,
        }),
        Err(LexerError::UnexpectedCharacter {
            character: '$',
            line: 1,
            col: 5,
        }),
        Err(LexerError::UnexpectedCharacter {
            character: '%',
            line: 1,
            col: 7,
        }),
        Err(LexerError::UnexpectedCharacter {
            character: '^',
            line: 1,
            col: 9,
        }),
        Err(LexerError::UnexpectedCharacter {
            character: '&',
            line: 1,
            col: 11,
        }),
        Err(LexerError::UnexpectedCharacter {
            character: '|',
            line: 1,
            col: 13,
        }),
        Err(LexerError::UnexpectedCharacter {
            character: '\\',
            line: 1,
            col: 15,
        }),
        Err(LexerError::UnexpectedCharacter {
            character: ':',
            line: 1,
            col: 17,
        }),
        Err(LexerError::UnexpectedCharacter {
            character: '\'',
            line: 1,
            col: 19,
        }),
        Err(LexerError::UnexpectedCharacter {
            character: '?',
            line: 1,
            col: 21,
        }),
    ];

    for (idx, (token, expected_token)) in tokens.into_iter().zip(expected).enumerate() {
        assert_eq!(token, expected_token, "index: {idx}");
    }
}

#[test]
fn can_scan_binary_groups() {
    let source = "(1 + 2) * (3 - 4)";
    let lexer = Lexer::new(source);

    let tokens = lexer.scan_all_tokens();
    let expected = [
        Ok(Token {
            token_type: TokenType::LeftParen,
            line: 1,
            col: 1,
        }),
        Ok(Token {
            token_type: TokenType::Number(1.0),
            line: 1,
            col: 2,
        }),
        Ok(Token {
            token_type: TokenType::Plus,
            line: 1,
            col: 4,
        }),
        Ok(Token {
            token_type: TokenType::Number(2.0),
            line: 1,
            col: 6,
        }),
        Ok(Token {
            token_type: TokenType::RightParen,
            line: 1,
            col: 7,
        }),
        Ok(Token {
            token_type: TokenType::Star,
            line: 1,
            col: 9,
        }),
        Ok(Token {
            token_type: TokenType::LeftParen,
            line: 1,
            col: 11,
        }),
        Ok(Token {
            token_type: TokenType::Number(3.0),
            line: 1,
            col: 12,
        }),
        Ok(Token {
            token_type: TokenType::Minus,
            line: 1,
            col: 14,
        }),
        Ok(Token {
            token_type: TokenType::Number(4.0),
            line: 1,
            col: 16,
        }),
        Ok(Token {
            token_type: TokenType::RightParen,
            line: 1,
            col: 17,
        }),
    ];

    for (idx, (token, expected_token)) in tokens.into_iter().zip(expected).enumerate() {
        assert_eq!(token, expected_token, "index: {idx}");
    }
}

#[test]
fn can_scan_variable_declaration() {
    let source = "var a = 1;";
    let lexer = Lexer::new(source);

    let tokens = lexer.scan_all_tokens();
    let expected = [
        Ok(Token {
            token_type: TokenType::Var,
            line: 1,
            col: 3,
        }),
        Ok(Token {
            token_type: TokenType::Identifier("a".to_owned()),
            line: 1,
            col: 5,
        }),
        Ok(Token {
            token_type: TokenType::Equal,
            line: 1,
            col: 7,
        }),
        Ok(Token {
            token_type: TokenType::Number(1.0),
            line: 1,
            col: 9,
        }),
        Ok(Token {
            token_type: TokenType::Semicolon,
            line: 1,
            col: 10,
        }),
    ];

    assert_eq!(tokens, expected);
}

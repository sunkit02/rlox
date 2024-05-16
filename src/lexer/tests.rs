use super::*;

#[test]
fn can_parse_rust_use_statement() {
    let statement = "use anyhow::{Context, Result};";
    let lexer = Lexer::new(statement);

    let tokens = lexer.scan_all_tokens();
    let expected_tokens = [
        Ok(Token {
            token_type: TokenType::Identifier,
            lexeme: "use".to_string(),
            line: 1,
            col: 3,
        }),
        Ok(Token {
            token_type: TokenType::Whitespace,
            lexeme: " ".to_string(),
            line: 1,
            col: 4,
        }),
        Ok(Token {
            token_type: TokenType::Identifier,
            lexeme: "anyhow".to_string(),
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
            lexeme: "{".to_string(),
            line: 1,
            col: 13,
        }),
        Ok(Token {
            token_type: TokenType::Identifier,
            lexeme: "Context".to_string(),
            line: 1,
            col: 20,
        }),
        Ok(Token {
            token_type: TokenType::Comma,
            lexeme: ",".to_string(),
            line: 1,
            col: 21,
        }),
        Ok(Token {
            token_type: TokenType::Whitespace,
            lexeme: " ".to_string(),
            line: 1,
            col: 22,
        }),
        Ok(Token {
            token_type: TokenType::Identifier,
            lexeme: "Result".to_string(),
            line: 1,
            col: 28,
        }),
        Ok(Token {
            token_type: TokenType::RightBrace,
            lexeme: "}".to_string(),
            line: 1,
            col: 29,
        }),
        Ok(Token {
            token_type: TokenType::Semicolon,
            lexeme: ";".to_string(),
            line: 1,
            col: 30,
        }),
    ];

    assert_eq!(tokens, expected_tokens);
}

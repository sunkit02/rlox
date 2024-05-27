use std::collections::HashMap;

use lazy_static::lazy_static;

use self::cursor::Peekable;
use self::token::TokenType;
use self::{cursor::Cursor, token::Token};

pub use self::error::{LexerError, Result};

pub mod cursor;
pub mod error;
pub mod token;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        map.insert("and", TokenType::And);
        map.insert("class", TokenType::Class);
        map.insert("else", TokenType::Else);
        map.insert("false", TokenType::False);
        map.insert("for", TokenType::For);
        map.insert("fun", TokenType::Fun);
        map.insert("if", TokenType::If);
        map.insert("nil", TokenType::Nil);
        map.insert("or", TokenType::Or);
        map.insert("print", TokenType::Print);
        map.insert("return", TokenType::Return);
        map.insert("super", TokenType::Super);
        map.insert("this", TokenType::This);
        map.insert("true", TokenType::True);
        map.insert("var", TokenType::Var);
        map.insert("while", TokenType::While);
        map
    };
}

#[derive(Debug)]
pub struct Lexer {
    source: Cursor,
    start: usize,
    current: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: Cursor::new(source),
            start: 0,
            current: 0,
            line: 1,
            col: 0,
        }
    }

    pub fn scan_token(&mut self) -> Option<Result<Token>> {
        self.start = self.current;

        let c = self.advance()?;
        let token_type_result = match c {
            // Single letter tokens
            '(' => Ok(TokenType::LeftParen),
            ')' => Ok(TokenType::RightParen),
            '{' => Ok(TokenType::LeftBrace),
            '}' => Ok(TokenType::RightBrace),
            ',' => Ok(TokenType::Comma),
            '.' => Ok(TokenType::Dot),
            '-' => Ok(TokenType::Minus),
            '+' => Ok(TokenType::Plus),
            ';' => Ok(TokenType::Semicolon),
            '*' => Ok(TokenType::Star),

            // Two-letter tokens
            '!' => {
                if self.source.peek() == Some('=') {
                    self.advance();
                    Ok(TokenType::BangEqual)
                } else {
                    Ok(TokenType::Bang)
                }
            }
            '=' => {
                if self.source.peek() == Some('=') {
                    self.advance();
                    Ok(TokenType::EqualEqual)
                } else {
                    Ok(TokenType::Equal)
                }
            }
            '<' => {
                if self.source.peek() == Some('=') {
                    self.advance();
                    Ok(TokenType::LessEqual)
                } else {
                    Ok(TokenType::Less)
                }
            }
            '>' => {
                if self.source.peek() == Some('=') {
                    self.advance();
                    Ok(TokenType::GreaterEqual)
                } else {
                    Ok(TokenType::Greater)
                }
            }

            // Multi-letter tokens
            '/' => {
                if self.source.peek() == Some('/') {
                    while self.source.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(TokenType::Comment)
                } else {
                    Ok(TokenType::Slash)
                }
            }
            '"' => self.handle_string_literal(),
            '0'..='9' => self.handle_numeric_literal(),
            'a'..='z' => self.handle_indentifier(),
            'A'..='Z' => self.handle_indentifier(),
            '_' => self.handle_indentifier(),

            // Whitespace
            ' ' | '\r' | '\t' => Ok(TokenType::Whitespace),
            '\n' => {
                // Increment line counter
                self.line += 1;
                // Reset col counter
                self.col = 0;

                Ok(TokenType::Whitespace)
            }

            unexpected => {
                self.start = self.current;
                Err(LexerError::UnexpectedCharacter {
                    character: unexpected,
                    line: self.line,
                    col: self.col,
                })
            }
        };

        let token_type = match token_type_result {
            Ok(token_type) => token_type,
            Err(e) => return Some(Err(e)),
        };

        // If the token is whitespace, simply pass over to the next token
        if let TokenType::Whitespace = token_type {
            self.scan_token()
        } else {
            Some(Ok(self.create_token(token_type)))
        }
    }

    #[inline]
    pub fn scan_all_tokens(self) -> Vec<Result<Token>> {
        self.into_iter().collect()
    }

    #[inline]
    pub fn is_at_end(&self) -> bool {
        self.source.is_at_end()
    }

    /// Advance needle of [Cursor] and corresponding bookkeeping of [Lexer]
    #[inline]
    fn advance(&mut self) -> Option<char> {
        let next_char = self.source.next();

        self.col += 1;
        self.current += 1;

        next_char
    }

    #[inline]
    fn create_token(&mut self, token_type: TokenType) -> Token {
        

        Token {
            token_type,
            line: self.line,
            col: self.col,
        }
    }

    #[inline]
    fn get_lexeme(&self) -> String {
        self.source
            .substring(self.start, self.current)
            .expect("positions `Lexer.start` and `Lexer.end` should always be valid")
    }

    #[inline]
    fn handle_string_literal(&mut self) -> Result<TokenType> {
        while self.source.peek() != Some('"') && !self.is_at_end() {
            if self.source.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            Err(LexerError::UnterminatedString {
                line: self.line,
                col: self.col,
            })
        } else {
            // The closing "
            self.advance();
            let lexeme = self.get_lexeme().chars().collect::<Vec<char>>();
            // trim surrounding quotes
            let literal = lexeme[1..lexeme.len() - 1].iter().collect::<String>();
            Ok(TokenType::String(literal))
        }
    }

    #[inline]
    fn handle_numeric_literal(&mut self) -> Result<TokenType> {
        while let Some(next_char) = self.source.peek() {
            if next_char.is_numeric() {
                self.advance();
            } else {
                break;
            }
        }

        if self.source.peek() == Some('.') {
            if let Some(char_after_dot) = self.source.peek_nth(1) {
                if char_after_dot.is_numeric() {
                    self.advance();

                    while let Some(next_char) = self.source.peek() {
                        if next_char.is_numeric() {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        let lexeme = self.get_lexeme();
        let number = lexeme
            .parse::<f64>()
            .map_err(|e| LexerError::FloatParsingError {
                lexeme,
                line: self.line,
                col: self.col,
                message: e.to_string(),
            })?;

        Ok(TokenType::Number(number))
    }

    #[inline]
    fn handle_indentifier(&mut self) -> Result<TokenType> {
        while let Some(next_char) = self.source.peek() {
            // Allow '_' as a seperator in identifiers
            if next_char.is_alphanumeric() || next_char == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let literal = self.get_lexeme();

        if let Some(keyword_type) = KEYWORDS.get(literal.as_str()) {
            Ok(keyword_type.clone())
        } else {
            Ok(TokenType::Identifier(literal))
        }
    }
}

impl Iterator for Lexer {
    type Item = self::error::Result<Token>;

    /// A direct wrapper call to [Lexer::scan_token]
    fn next(&mut self) -> Option<Self::Item> {
        self.scan_token()
    }
}

#[cfg(test)]
mod tests;

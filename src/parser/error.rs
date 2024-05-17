use std::fmt::Display;

use crate::lexer::token::{Token, TokenType};

pub type Result<T> = std::result::Result<T, ParserError>;

#[derive(Debug)]
pub enum ParserError {
    AccessedTokenDoesNotExist,
    InvalidTokenToOperatorConversion(Token),
    InvalidPrimaryExpressionToken(Token),
    MissingExpectedToken {
        token_type: TokenType,
        message: String,
    },
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ParserError {}

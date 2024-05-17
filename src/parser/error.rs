use thiserror::Error;

use crate::lexer::token::{Token, TokenType};

pub type Result<T> = std::result::Result<T, ParserError>;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("unexpected end of tokens")]
    UnexpectedEndOfTokens,

    #[error("{0} is not a valid operator token")]
    InvalidTokenToOperatorConversion(Token),

    #[error("expected expression, got: {0}")]
    InvalidPrimaryExpressionToken(Token),

    #[error("expected {}: {}",.token_type.to_str(), .message)]
    MissingExpectedToken {
        token_type: TokenType,
        message: String,
    },
}

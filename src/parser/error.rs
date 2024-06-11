use thiserror::Error;

use crate::lexer::token::{Token, TokenType};

pub type Result<T> = std::result::Result<T, ParserError>;

#[derive(Debug, Error, PartialEq)]
pub enum ParserError {
    #[error("unexpected end of tokens")]
    UnexpectedEndOfTokens,

    #[error("{0} is not a valid operator token")]
    InvalidTokenToOperatorConversion(Token),

    // TODO: Try to give more context to what lead to this error. Ex. a block missing opening brace
    // will return this error. How can we indicate that?
    #[error("expected expression, got: {0}")]
    InvalidPrimaryExpressionToken(Token),

    #[error("invalid assignment target at: {0}")]
    InvalidAssignmentTarget(Token),

    // TODO: Try to include line and column info when reporting `MissingExpectedToken` error.
    #[error("expected {}: {}",.token_type.name(), .message)]
    MissingExpectedToken {
        token_type: TokenType,
        message: String,
    },

    #[error("unexpected {}, expected {}", .got, .expected)]
    UnexpectedLanguageComponent { expected: String, got: String },
}

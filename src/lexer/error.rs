use std::fmt::Display;

pub type Result<T> = std::result::Result<T, LexerError>;

#[derive(Debug, PartialEq)]
pub enum LexerError {
    UnterminatedString {
        line: usize,
        col: usize,
    },
    UnexpectedCharacter {
        character: char,
        line: usize,
        col: usize,
    },
    FloatParsingError {
        lexeme: String,
        line: usize,
        col: usize,
        message: String,
    },
}

impl LexerError {
    fn name(&self) -> &'static str {
        match self {
            LexerError::UnterminatedString { .. } => "UnterminatedString",
            LexerError::UnexpectedCharacter { .. } => "UnexpectedCharacter",
            LexerError::FloatParsingError { .. } => "FloatParsingError",
        }
    }
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (line, col, error_name, msg) = match self {
            LexerError::UnterminatedString { line, col } => {
                (line, col, self.name(), "".to_string())
            }
            LexerError::UnexpectedCharacter {
                character,
                line,
                col,
            } => (line, col, self.name(), character.to_string()),
            LexerError::FloatParsingError {
                lexeme,
                line,
                col,
                message,
            } => (line, col, self.name(), format!("{}, {}", message, lexeme)),
        };

        write!(f, "[line {}: col {}] {}: {}", line, col, error_name, msg)
    }
}

impl std::error::Error for LexerError {}

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub col: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let literal = match &self.token_type {
            TokenType::String(str_literal) => str_literal.to_string(),
            TokenType::Number(num_literal) => num_literal.to_string(),
            _ => "".to_string(),
        };
        write!(f, "{} {}", self.token_type.name(), literal)
    }
}

impl Token {
    pub fn is_identifier(&self) -> bool {
        matches!(self.token_type, TokenType::Identifier(_))
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    // Ignored
    Comment,
    Whitespace,
    Eof,
}

impl TokenType {
    /// Returns the name of the variant as a string slice
    pub fn name(&self) -> &str {
        match self {
            TokenType::LeftParen => "LeftParen",
            TokenType::RightParen => "RightParen",
            TokenType::LeftBrace => "LeftBrace",
            TokenType::RightBrace => "RightBrace",
            TokenType::Comma => "Comma",
            TokenType::Dot => "Dot",
            TokenType::Minus => "Minus",
            TokenType::Plus => "Plus",
            TokenType::Semicolon => "Semicolon",
            TokenType::Slash => "Slash",
            TokenType::Star => "Star",
            TokenType::Bang => "Bang",
            TokenType::BangEqual => "BangEqual",
            TokenType::Equal => "Equal",
            TokenType::EqualEqual => "EqualEqual",
            TokenType::Greater => "Greater",
            TokenType::GreaterEqual => "GreaterEqual",
            TokenType::Less => "Less",
            TokenType::LessEqual => "LessEqual",
            TokenType::Identifier(_) => "Identifier",
            TokenType::String(_) => "String",
            TokenType::Number(_) => "Number",
            TokenType::And => "And",
            TokenType::Class => "Class",
            TokenType::Else => "Else",
            TokenType::False => "False",
            TokenType::Fun => "Fun",
            TokenType::For => "For",
            TokenType::If => "If",
            TokenType::Nil => "Nil",
            TokenType::Or => "Or",
            TokenType::Print => "Print",
            TokenType::Return => "Return",
            TokenType::Super => "Super",
            TokenType::This => "This",
            TokenType::True => "True",
            TokenType::Var => "Var",
            TokenType::While => "While",
            TokenType::Comment => "Comment",
            TokenType::Whitespace => "Whitespace",
            TokenType::Eof => "Eof",
        }
    }
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}

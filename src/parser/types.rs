use std::fmt::Display;

use crate::lexer::token::{Token, TokenType};

use super::error::ParserError;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(Expr),
    Print(Expr),
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    // TODO: Do these later.
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Operator,
        right: Box<Expr>,
    },
    Grouping {
        inner: Box<Expr>,
    },
    Literal {
        value: Value,
    },
    Unary {
        operator: Operator,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}

/// Types of valid values in the Lox language
#[derive(Debug, PartialEq)]
pub enum Value {
    Boolean(bool),
    Nil,
    Number(f64),
    String(String),
}

#[derive(Debug, PartialEq)]
pub struct Operator {
    pub operator_type: OperatorType,
    /// Line number in source file
    pub src_line: usize,
    /// Column number in source file
    pub src_col: usize,
}

#[derive(Debug, PartialEq)]
pub enum OperatorType {
    Dot,

    Minus,
    Plus,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl TryFrom<Token> for Operator {
    type Error = ParserError;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        let operator_type = match token.token_type {
            TokenType::Dot => OperatorType::Dot,
            TokenType::Minus => OperatorType::Minus,
            TokenType::Plus => OperatorType::Plus,
            TokenType::Slash => OperatorType::Slash,
            TokenType::Star => OperatorType::Star,
            TokenType::Bang => OperatorType::Bang,
            TokenType::BangEqual => OperatorType::BangEqual,
            TokenType::Equal => OperatorType::Equal,
            TokenType::EqualEqual => OperatorType::EqualEqual,
            TokenType::Greater => OperatorType::Greater,
            TokenType::GreaterEqual => OperatorType::GreaterEqual,
            TokenType::Less => OperatorType::Less,
            TokenType::LessEqual => OperatorType::LessEqual,
            _ => return Err(ParserError::InvalidTokenToOperatorConversion(token)),
        };

        Ok(Self {
            operator_type,
            src_line: token.line,
            src_col: token.col,
        })
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_string = match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => format!("({operator} {left} {right})"),
            Expr::Grouping { inner } => format!("(group {inner})"),
            Expr::Literal { value } => format!("{value}"),
            Expr::Unary { operator, right } => format!("({operator} {right})"),
            Expr::Assign { name, value } => format!("(assign {name} <- {value})"),
            Expr::Variable { name } => format!("(var {name})"),
        };

        write!(f, "{formatted_string}")
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self.operator_type {
            OperatorType::Dot => ".",
            OperatorType::Minus => "-",
            OperatorType::Plus => "+",
            OperatorType::Slash => "/",
            OperatorType::Star => "*",
            OperatorType::Bang => "!",
            OperatorType::BangEqual => "!=",
            OperatorType::Equal => "=",
            OperatorType::EqualEqual => "==",
            OperatorType::Greater => ">",
            OperatorType::GreaterEqual => ">=",
            OperatorType::Less => "<",
            OperatorType::LessEqual => "<=",
        };

        write!(f, "{string}")
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Value::Boolean(boolean) => boolean.to_string(),
            Value::Nil => "nil".to_string(),
            // Display integer floats without the decimal point
            Value::Number(number) => {
                if number.fract() == 0.0 {
                    (*number as u64).to_string()
                } else {
                    number.to_string()
                }
            }
            Value::String(string) => format!("\"{string}\""),
        };

        write!(f, "{string}")
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Stmt::Block(expressions) => {
                let string = expressions.iter().fold(String::new(), |mut acc, stmt| {
                    acc.push_str(&format!("{stmt} "));
                    acc
                });

                format!("{{ {string} }}")
            }
            Stmt::Expression(expr) => format!("{expr};"),
            Stmt::Print(expr) => format!("(print {expr});"),
            Stmt::Var { name, initializer } => format!(
                "(var {name} = {});",
                if let Some(initializer) = initializer {
                    initializer.to_string()
                } else {
                    "nil".to_owned()
                }
            ),
        };

        write!(f, "{string}")
    }
}

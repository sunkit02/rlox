use thiserror::Error;

use crate::{
    lexer::token::Token,
    parser::types::{Operator, Value},
};

use super::environment::EnvironmentError;

pub type Result<T> = std::result::Result<T, RuntimeError>;

#[derive(Debug, Error, PartialEq)]
pub enum RuntimeError {
    #[error("variable '{}' is already defined.", .name)]
    VariableAlreadyDefined {
        /// Name of the variable
        name: String,
        /// Line of the new declaration
        line: usize,
        /// Column of the new declaration
        col: usize,
    },

    #[error("undefined variable '{}'", .name)]
    UndefinedVariable {
        /// Name of the variable
        name: String,
        /// Line of the new declaration
        line: usize,
        /// Column of the new declaration
        col: usize,
    },

    #[error("cannot assign a value to {}", .0.token_type.name())]
    InvalidAssignTarget(Token),

    #[error("invalid operands for '{}', expected {}", .operator, .expected)]
    InvalidOperands {
        operator: Operator,
        expected: String,
    },

    #[error("invalid operator '{}'", .0)]
    InvalidUnaryOperator(Operator),

    #[error("invalid operator '{}' for value {}", .operator, .value)]
    InvalidUnaryOperatorForValue { operator: Operator, value: Value },
}

impl RuntimeError {
    pub fn from_env_err(env_err: EnvironmentError, name_token: Token) -> Self {
        match env_err {
            EnvironmentError::VariableAlreadyDefined(name) => Self::VariableAlreadyDefined {
                name,
                line: name_token.line,
                col: name_token.col,
            },
            EnvironmentError::UndefinedVariable(name) => Self::UndefinedVariable {
                name,
                line: name_token.line,
                col: name_token.col,
            },
            EnvironmentError::ExitingGlobalScope => {
                panic!("The interpreter should never try to exit the global scope.")
            }
        }
    }
}

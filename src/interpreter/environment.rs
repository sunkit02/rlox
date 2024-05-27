use std::collections::HashMap;

use thiserror::Error;

use crate::parser::types::Value;

pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn new_with_enclosing(enclosing: Environment) -> Self {
        Self {
            enclosing: Some(Box::new(enclosing)),
            values: HashMap::new(),
        }
    }

    /// Creates a new variable `name` and assign `value` to it. Returns an `Err` if the variable
    /// already exists.
    pub fn define(&mut self, name: String, value: Value) -> Result<(), EnvironmentError> {
        if self.values.contains_key(&name) {
            return Err(EnvironmentError::VariableAlreadyDefined(name));
        }

        self.values.insert(name, value);

        Ok(())
    }

    /// Assigns `value` to an existing variable. Returns `value` if successful and an `Err` if the
    /// variable doesn't exist.
    pub fn assign(&mut self, name: String, value: Value) -> Result<(), EnvironmentError> {
        if !self.values.contains_key(&name) {
            return Err(EnvironmentError::UndefinedVariable(name));
        }

        self.values.insert(name, value);

        Ok(())
    }

    pub fn get(&self, name: &String) -> Result<&Value, EnvironmentError> {
        self.values
            .get(name)
            .ok_or(EnvironmentError::UndefinedVariable(name.to_owned()))
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            enclosing: None,
            values: HashMap::default(),
        }
    }
}

#[derive(Debug, Error)]
pub enum EnvironmentError {
    #[error("variable '{}' is already defined", .0)]
    VariableAlreadyDefined(String),

    #[error("undefined variable '{}'", .0)]
    UndefinedVariable(String),
}

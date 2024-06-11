use std::{collections::HashMap, mem};

use thiserror::Error;

use crate::parser::types::Value;

#[derive(Default)]
/// This encapsulates a "scope". Like the global scope, the scope inside a function, etc.
pub struct Environment {
    /// The enclosing/parent scope of the current scope, or the scope that is one level higher than
    /// the current scope. The global scope will not have an enclosing scope.
    enclosing: Option<Box<Environment>>,

    /// All the variables contained in the current scope.
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
    ///
    /// # Errors
    ///
    /// This method returns an error when the variable `name` has already been defined.
    pub fn define(&mut self, name: String, value: Value) -> Result<(), EnvironmentError> {
        if self.values.contains_key(&name) {
            return Err(EnvironmentError::VariableAlreadyDefined(name));
        }

        self.values.insert(name, value);

        Ok(())
    }

    /// Assigns `value` to an existing variable. Returns `value` if successful and an `Err` if the
    /// variable doesn't exist.
    ///
    /// # Errors
    ///
    /// This method returns an error when the variable `name` has not been defined in the current scope
    /// or any of its enclosing scopes.
    pub fn assign(&mut self, name: String, value: Value) -> Result<(), EnvironmentError> {
        fn assign_recur(
            env: &mut dyn AsMut<Environment>,
            name: String,
            value: Value,
        ) -> Result<(), EnvironmentError> {
            let env = env.as_mut();

            if !env.values.contains_key(&name) {
                if let Some(ref mut enclosing) = env.enclosing {
                    return assign_recur(enclosing, name, value);
                } else {
                    return Err(EnvironmentError::UndefinedVariable(name));
                }
            }

            env.values.insert(name, value);
            Ok(())
        }

        assign_recur(self, name, value)
    }

    /// Returns a reference to the value of the variable `name`.
    ///
    /// # Errors
    ///
    /// This method returns an error when the variable `name` has not been defined in the current scope
    /// or any of its enclosing scopes.
    pub fn get(&self, name: &String) -> Result<&Value, EnvironmentError> {
        fn get_recur<'a>(
            env: &'a dyn AsRef<Environment>,
            name: &String,
        ) -> Result<&'a Value, EnvironmentError> {
            let env = env.as_ref();

            if !env.values.contains_key(name) {
                if let Some(ref enclosing) = env.enclosing {
                    return get_recur(enclosing, name);
                } else {
                    return Err(EnvironmentError::UndefinedVariable(name.to_owned()));
                }
            }

            env.values
                .get(name)
                .ok_or_else(|| EnvironmentError::UndefinedVariable(name.to_owned()))
        }

        get_recur(self, name)
    }

    /// Creates a new scope by replacing the current `self` with a new `Environment` scope and
    /// setting the current `self` as the `enclosing` of the new scope.
    pub fn enter_new_scope(&mut self) {
        // This works because `mem::take` replaces the original value with its default
        // values which is what we want with a new scope.
        let enclosing = mem::take(self);

        // Set the "current" scope as the enclosing of the newly created scope
        self.enclosing = Some(Box::new(enclosing));
    }

    /// Exits the current scope, sets its enclosing scope as the current scope, and returns the
    /// previously current scope.
    ///
    /// # Errors
    ///
    /// This method returns an error when it is called on the global scope, in other words, when
    /// the `enclosing` field is `None`
    pub fn exit_current_scope(&mut self) -> Result<Environment, EnvironmentError> {
        // Extract the enclosing scope or return an error if there is none.
        let enclosing = self
            .enclosing
            .take()
            .map(|environemnt| *environemnt)
            .ok_or(EnvironmentError::ExitingGlobalScope)?;

        // Replace the current scope with the enclosing scope and return the current scope.
        Ok(mem::replace(self, enclosing))
    }
}

#[derive(Debug, Error)]
pub enum EnvironmentError {
    #[error("variable '{}' is already defined", .0)]
    VariableAlreadyDefined(String),

    #[error("undefined variable '{}'", .0)]
    UndefinedVariable(String),

    #[error("cannot exit the global scope")]
    ExitingGlobalScope,
}

impl AsRef<Environment> for Environment {
    fn as_ref(&self) -> &Environment {
        self
    }
}

impl AsMut<Environment> for Environment {
    fn as_mut(&mut self) -> &mut Environment {
        self
    }
}

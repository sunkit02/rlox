use crate::{
    lexer::token::TokenType,
    parser::types::{Expr, Operator, OperatorType, Stmt, Value},
};

use self::environment::Environment;
use error::{Result, RuntimeError};

pub mod environment;
pub mod error;

pub trait ErrorReporter {
    fn report_err(&self, error: &RuntimeError);
}

pub struct Interpreter {
    environment: Environment,
    error_reporters: Vec<Box<dyn ErrorReporter>>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::default(),
            error_reporters: Vec::new(),
        }
    }

    pub fn with_reporters<I>(reporters: I) -> Self
    where
        I: IntoIterator<Item = Box<dyn ErrorReporter>>,
    {
        Self {
            environment: Environment::default(),
            error_reporters: reporters.into_iter().collect(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for stmt in statements {
            if let Err(e) = self.execute(stmt) {
                self.error_reporters
                    .iter()
                    .for_each(|reporter| reporter.report_err(&e))
            }
        }
    }

    pub fn execute(&mut self, stmt: Stmt) -> Result<()> {
        match stmt {
            Stmt::Block(stmts) => {
                self.environment.enter_new_scope();

                for stmt in stmts {
                    self.execute(stmt)?;
                }

                self.environment
                    .exit_current_scope()
                    .expect("should never fail to exit a newly entered scope");
            }
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
            }
            Stmt::Print(expr) => println!("{}", self.evaluate(expr)?.stringify()),
            Stmt::Var {
                name: name_token,
                initializer,
            } => {
                let TokenType::Identifier(name) = name_token.clone().token_type else {
                    panic!("`name` field in `Stmt::Var` should always be an identifier");
                };

                let initial_value = initializer
                    .map(|expr| self.evaluate(expr))
                    .transpose()?
                    .unwrap_or(Value::Nil); // Uninitialized variables default to `nil`

                self.environment
                    .define(name, initial_value)
                    .map_err(|env_err| RuntimeError::from_env_err(env_err, name_token))?;
            }
        }

        Ok(())
    }

    fn evaluate(&mut self, expr: Expr) -> Result<Value> {
        let value = match expr {
            Expr::Assign {
                name: name_token,
                value,
            } => {
                let name = {
                    let TokenType::Identifier(ref name) = name_token.token_type else {
                        return Err(RuntimeError::InvalidAssignTarget(name_token));
                    };

                    name
                };

                let value = self.evaluate(*value)?;
                self.environment
                    .assign(name.to_owned(), value.clone())
                    .map_err(|env_err| RuntimeError::from_env_err(env_err, name_token))?;

                value
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate(*left)?;
                let right = self.evaluate(*right)?;

                self.evaluate_binary_expression(left, right, operator)?
            }
            Expr::Grouping { inner } => self.evaluate(*inner)?,
            Expr::Literal { value } => value,
            Expr::Unary { operator, right } => self.evaluate_unary_expression(operator, *right)?,
            Expr::Variable { name: name_token } => {
                let name = {
                    let TokenType::Identifier(ref name) = name_token.token_type else {
                        panic!("name token for `Expr::Variable` should always be an identifier");
                    };

                    name
                };

                self.environment
                    .get(name)
                    .cloned()
                    .map_err(|env_err| RuntimeError::from_env_err(env_err, name_token))?
            }
        };

        Ok(value)
    }

    fn evaluate_unary_expression(&mut self, operator: Operator, rhs: Expr) -> Result<Value> {
        match operator.operator_type {
            OperatorType::Minus => {
                let rhs = self.evaluate(rhs)?;

                if let Value::Number(number) = rhs {
                    Ok(Value::Number(-number))
                } else {
                    Err(RuntimeError::InvalidUnaryOperatorForValue {
                        operator,
                        value: rhs,
                    })
                }
            }
            OperatorType::Bang => todo!(),

            // Illegal unary operators (for now)
            _ => Err(RuntimeError::InvalidUnaryOperator(operator)),
        }
    }

    fn evaluate_binary_expression(
        &self,
        left: Value,
        right: Value,
        operator: Operator,
    ) -> Result<Value> {
        let value = match operator.operator_type {
            OperatorType::Minus => match (left, right) {
                (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs - rhs),
                _ => {
                    return Err(RuntimeError::InvalidOperands {
                        operator,
                        expected: "two numbers".to_owned(),
                    })
                }
            },
            OperatorType::Plus => match (left, right) {
                (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs + rhs),

                // Allow implicit string conversions
                (lhs, rhs) => {
                    let mut lhs = lhs.stringify();
                    lhs.push_str(rhs.stringify().as_str());
                    Value::String(lhs)
                }
            },
            OperatorType::Slash => match (left, right) {
                (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs / rhs),
                _ => {
                    return Err(RuntimeError::InvalidOperands {
                        operator,
                        expected: "two numbers".to_owned(),
                    })
                }
            },
            OperatorType::Star => match (left, right) {
                (Value::Number(lhs), Value::Number(rhs)) => Value::Number(lhs * rhs),
                _ => {
                    return Err(RuntimeError::InvalidOperands {
                        operator,
                        expected: "two numbers".to_owned(),
                    })
                }
            },

            OperatorType::BangEqual => Value::Boolean(left != right),
            OperatorType::EqualEqual => Value::Boolean(left == right),
            OperatorType::Greater => match (left, right) {
                (Value::Number(lhs), Value::Number(rhs)) => Value::Boolean(lhs > rhs),
                _ => {
                    return Err(RuntimeError::InvalidOperands {
                        operator,
                        expected: "two numbers".to_owned(),
                    })
                }
            },
            OperatorType::GreaterEqual => match (left, right) {
                (Value::Number(lhs), Value::Number(rhs)) => Value::Boolean(lhs >= rhs),
                _ => {
                    return Err(RuntimeError::InvalidOperands {
                        operator,
                        expected: "two numbers".to_owned(),
                    })
                }
            },
            OperatorType::Less => match (left, right) {
                (Value::Number(lhs), Value::Number(rhs)) => Value::Boolean(lhs < rhs),
                _ => {
                    return Err(RuntimeError::InvalidOperands {
                        operator,
                        expected: "two numbers".to_owned(),
                    })
                }
            },
            OperatorType::LessEqual => match (left, right) {
                (Value::Number(lhs), Value::Number(rhs)) => Value::Boolean(lhs <= rhs),
                _ => {
                    return Err(RuntimeError::InvalidOperands {
                        operator,
                        expected: "two numbers".to_owned(),
                    })
                }
            },

            // Invalid operators in this situation
            OperatorType::Equal => {
                panic!("Should never get '=' as an operator between two values in this state")
            }
            OperatorType::Bang => {
                panic!("Should never get '!' as an operator between two values in this state")
            }

            // Todos
            OperatorType::Dot => todo!("Used when implementing classes, fields, and methods"),
        };

        Ok(value)
    }
}

#[cfg(test)]
mod tests;

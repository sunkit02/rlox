use crate::lexer::token::{
    Token,
    TokenType::{self, *},
};

use self::types::{Expr, Stmt, Value};
use self::{
    error::{ParserError, Result},
    types::Operator,
};

pub mod environment;
pub mod error;
pub mod types;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new<I: IntoIterator<Item = Token>>(tokens: I) -> Self {
        Self {
            tokens: tokens.into_iter().collect(),
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Stmt> {
        if self.matches_any([Var]) {
            self.advance().ok_or(ParserError::UnexpectedEndOfTokens)?;

            debug_assert!(
                self.peek()
                    .map(|token| if let TokenType::Identifier(_) = token.token_type {
                        true
                    } else {
                        false
                    })
                    == Some(true)
            );

            self.var_declaration().inspect_err(|_| self.synchronize())
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        // TODO: Fix this ugly little hack to get Identifiers to work.
        // The PartialEq impl for TokenType should not be broken and ignore the
        // value held by the variant.
        let name = self.consume(Identifier("".to_owned()), "expected variable name")?;

        let initializer = if self.matches_any([Equal]) {
            self.advance().ok_or(ParserError::UnexpectedEndOfTokens)?;
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(Semicolon, "expected ';' after variable declaration")?;

        Ok(Stmt::Var { name, initializer })
    }

    fn statement(&mut self) -> Result<Stmt> {
        let current_token = self.peek().ok_or(ParserError::UnexpectedEndOfTokens)?;
        match current_token.token_type {
            Print => self.print_statement(),
            LeftBrace => self.block(),
            _ => self.expression_statement(),
        }
    }

    fn block(&mut self) -> Result<Stmt> {
        self.consume(LeftBrace, "expected '{' at start of block")?;
        let mut statements = Vec::new();

        while !self.matches_any([RightBrace]) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(RightBrace, "expected '}' at end of block")?;

        Ok(Stmt::Block(statements))
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        self.consume(Print, "expected a `print` keyword")?;
        let expr = self.expression()?;
        self.consume(Semicolon, "expected ';' after value")?;

        Ok(Stmt::Print(expr))
    }

    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(Semicolon, "expected ';' after expression")?;

        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr> {
        let expr = self.equality()?;

        if self.matches_any([Equal]) {
            let equals_token = self
                .advance()
                .cloned()
                .ok_or(ParserError::UnexpectedEndOfTokens)?;

            let value = self.assignment()?;

            if let Expr::Variable { name } = expr {
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            }

            return Err(ParserError::InvalidAssignmentTarget(equals_token));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        while self.matches_any([BangEqual, EqualEqual]) {
            let operator_token = self
                .advance()
                .cloned()
                .ok_or(ParserError::UnexpectedEndOfTokens)?;

            let operator = Operator::try_from(operator_token)?;

            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;

        while self.matches_any([Less, LessEqual, Greater, GreaterEqual]) {
            let operator_token = self
                .advance()
                .cloned()
                .ok_or(ParserError::UnexpectedEndOfTokens)?;

            let operator = Operator::try_from(operator_token)?;

            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;

        while self.matches_any([Minus, Plus]) {
            let operator_token = self
                .advance()
                .cloned()
                .ok_or(ParserError::UnexpectedEndOfTokens)?;

            let operator = Operator::try_from(operator_token)?;

            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;

        while self.matches_any([Slash, Star]) {
            let operator_token = self
                .advance()
                .cloned()
                .ok_or(ParserError::UnexpectedEndOfTokens)?;

            let operator = Operator::try_from(operator_token)?;

            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.matches_any([Bang, Minus]) {
            let operator_token = self
                .advance()
                .cloned()
                .ok_or(ParserError::UnexpectedEndOfTokens)?;

            let operator = Operator::try_from(operator_token)?;
            let right = self.primary()?;

            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr> {
        let token = self
            .advance()
            .cloned()
            .ok_or(ParserError::UnexpectedEndOfTokens)?;

        let expr = match token.token_type {
            Nil => Expr::Literal { value: Value::Nil },
            False => Expr::Literal {
                value: Value::Boolean(false),
            },
            True => Expr::Literal {
                value: Value::Boolean(true),
            },
            String(str) => Expr::Literal {
                value: Value::String(str),
            },
            Number(num) => Expr::Literal {
                value: Value::Number(num),
            },
            LeftParen => {
                let inner_expr = self.expression()?;
                self.consume(RightParen, "expected ')' after expression.")?;
                Expr::Grouping {
                    inner: Box::new(inner_expr),
                }
            }
            Identifier(_) => Expr::Variable { name: token },

            _ => return Err(ParserError::InvalidPrimaryExpressionToken(token)),
        };

        Ok(expr)
    }
}

// Helper functions
impl Parser {
    fn matches_any<I: IntoIterator<Item = TokenType>>(&self, tokens: I) -> bool {
        if self.is_at_end() {
            return false;
        }

        let current_token = self.peek().expect("current token should exist");

        tokens
            .into_iter()
            .any(|target_token_type| current_token.token_type == target_token_type)
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.previous();
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.current - 1)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn consume(&mut self, token_type: TokenType, error_message: &str) -> Result<Token> {
        let missing_token_error = ParserError::MissingExpectedToken {
            token_type: token_type.clone(),
            message: error_message.to_owned(),
        };

        let current_token = match self.peek() {
            Some(token) => token,
            None => return Err(missing_token_error),
        };

        if current_token.token_type == token_type {
            let current_token = self.advance().ok_or(ParserError::UnexpectedEndOfTokens)?;
            Ok(current_token.clone())
        } else {
            Err(missing_token_error)
        }
    }

    /// Escape all tokens until the next class, function, variable declaration, or for, if , while,
    /// print, return statement, or semilcolon
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            let previous = self.previous().expect("previous token should exist");
            if previous.token_type == TokenType::Semicolon {
                return;
            }

            let current = self.peek().expect("current token should exist");
            match current.token_type {
                Class | Fun | Var | For | If | While | Print | Return => return,
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests;

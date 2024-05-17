use crate::lexer::token::{
    Token,
    TokenType::{self, *},
};

use self::types::{Expr, Value};
use self::{
    error::{ParserError, Result},
    types::Operator,
};

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

    pub fn parse(&mut self) -> Result<Expr> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        while self.matches_any([BangEqual, EqualEqual]) {
            let operator_token = self
                .advance()
                .cloned()
                .ok_or(ParserError::AccessedTokenDoesNotExist)?;

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
                .ok_or(ParserError::AccessedTokenDoesNotExist)?;

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
                .ok_or(ParserError::AccessedTokenDoesNotExist)?;

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
                .ok_or(ParserError::AccessedTokenDoesNotExist)?;

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
                .ok_or(ParserError::AccessedTokenDoesNotExist)?;

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
            .ok_or(ParserError::AccessedTokenDoesNotExist)?;

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
                self.consume(RightParen, "Expect ')' after expression.")?;
                Expr::Grouping {
                    inner: Box::new(inner_expr),
                }
            }

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

    fn consume(&mut self, token_type: TokenType, error_message: &str) -> Result<()> {
        let current = self.peek().ok_or(ParserError::AccessedTokenDoesNotExist)?;
        if current.token_type != token_type {
            return Err(ParserError::MissingExpectedToken {
                token_type,
                message: error_message.to_owned(),
            });
        }

        Ok(())
    }
}

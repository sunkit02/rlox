use pretty_assertions::assert_eq;

use crate::{
    lexer::{
        error::Result,
        token::{Token, TokenType},
        Lexer,
    },
    parser::error::ParserError,
};

use super::{
    types::{Expr, Operator, OperatorType, Stmt, Value},
    Parser,
};

/// Tokenize a string of lox source code provided by `src`.
///
/// # Panic
/// Panics if the source code provided has syntax errors.
fn tokenize(src: &str) -> Vec<Token> {
    Lexer::new(src)
        .scan_all_tokens()
        .into_iter()
        .collect::<Result<Vec<Token>>>()
        .expect("source code should be valid")
}

#[test]
fn can_parse_block_statement() {
    let tokens = tokenize(
        r#"{
            1 + 2;
            "Hello, " + "world!";
        }"#,
    );

    let statements = Parser::new(tokens).parse().unwrap();
    let expected = Stmt::Block(Vec::from_iter([
        Stmt::Expression(Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Value::Number(1.0),
            }),
            operator: Operator {
                operator_type: OperatorType::Plus,
                src_line: 2,
                src_col: 15,
            },
            right: Box::new(Expr::Literal {
                value: Value::Number(2.0),
            }),
        }),
        Stmt::Expression(Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Value::String("Hello, ".to_owned()),
            }),
            operator: Operator {
                operator_type: OperatorType::Plus,
                src_line: 3,
                src_col: 23,
            },
            right: Box::new(Expr::Literal {
                value: Value::String("world!".to_owned()),
            }),
        }),
    ]));
    let expected = [expected];

    assert_eq!(statements, expected);
}

#[test]
fn error_on_missing_closing_brace_for_block_statement() {
    let tokens = tokenize("{ 1 + 2; 3 * 4;");

    let result = Parser::new(tokens).parse();
    let expected = Err(ParserError::MissingExpectedToken {
        token_type: TokenType::RightBrace,
        message: "expected '}' at end of block".to_owned(),
    });

    assert_eq!(result, expected);
}

#[test]
fn error_on_stray_opening_brace() {
    let tokens = tokenize("1 + 2; 3 * 4; }");

    let result = Parser::new(tokens).parse();
    let expected = Err(ParserError::InvalidPrimaryExpressionToken(Token {
        token_type: TokenType::RightBrace,
        line: 1,
        col: 15,
    }));

    assert_eq!(result, expected);
}

#[test]
fn can_parse_expression_statement() {
    let tokens = tokenize("1 + 2;");

    let result = Parser::new(tokens).parse().unwrap();
    let expected = [Stmt::Expression(Expr::Binary {
        left: Box::new(Expr::Literal {
            value: Value::Number(1.0),
        }),
        operator: Operator {
            operator_type: OperatorType::Plus,
            src_line: 1,
            src_col: 3,
        },
        right: Box::new(Expr::Literal {
            value: Value::Number(2.0),
        }),
    })];

    assert_eq!(result, expected);
}

#[test]
fn error_on_missing_semicolon_for_expression_statement() {
    let tokens = tokenize("1 + 2");

    let result = Parser::new(tokens).parse();
    let expected = Err(ParserError::MissingExpectedToken {
        token_type: TokenType::Semicolon,
        message: "expected ';' after expression".to_owned(),
    });

    assert_eq!(result, expected);
}

#[test]
fn can_parse_print_statement() {
    let tokens = tokenize("print \"Hello, world!\";");

    let result = Parser::new(tokens).parse().unwrap();
    let expected = [Stmt::Print(Expr::Literal {
        value: Value::String("Hello, world!".to_owned()),
    })];

    assert_eq!(result, expected);
}

#[test]
fn can_parse_var_statement_with_initializer() {
    let tokens = tokenize("var a = 1;");

    let result = Parser::new(tokens).parse().unwrap();
    let expected = [Stmt::Var {
        name: Token {
            token_type: TokenType::Identifier("a".to_owned()),
            line: 1,
            col: 5,
        },
        initializer: Some(Expr::Literal {
            value: Value::Number(1.0),
        }),
    }];

    assert_eq!(result, expected);
}

#[test]
fn can_parse_var_statement_without_initializer() {
    let tokens = tokenize("var a;");

    let result = Parser::new(tokens).parse().unwrap();
    let expected = [Stmt::Var {
        name: Token {
            token_type: TokenType::Identifier("a".to_owned()),
            line: 1,
            col: 5,
        },
        initializer: None,
    }];

    assert_eq!(result, expected);
}

#[test]
fn can_parse_assign_expression() {
    let tokens = tokenize("b = 21 / 7;");

    let result = Parser::new(tokens).parse().unwrap();
    let expected = [Stmt::Expression(Expr::Assign {
        name: Token {
            token_type: TokenType::Identifier("b".to_owned()),
            line: 1,
            col: 1,
        },
        value: Box::new(Expr::Binary {
            left: Box::new(Expr::Literal {
                value: Value::Number(21.0),
            }),
            operator: Operator {
                operator_type: OperatorType::Slash,
                src_line: 1,
                src_col: 8,
            },
            right: Box::new(Expr::Literal {
                value: Value::Number(7.0),
            }),
        }),
    })];

    assert_eq!(result, expected);
}

#[test]
fn error_on_invalid_assignment_target() {
    // Should not be able to assign a value to an expression evaluating into a boolean
    let tokens = tokenize("false == true = 5;");

    let result = Parser::new(tokens).parse();
    let expected = Err(ParserError::InvalidAssignmentTarget(Token {
        token_type: TokenType::Equal,
        line: 1,
        col: 15,
    }));

    assert_eq!(result, expected);
}

#[test]
fn can_parse_binary_expression() {
    let tokens = tokenize("21 / 7");

    let mut parser = Parser::new(tokens);
    let result = parser.expression();

    let expected = Ok(Expr::Binary {
        left: Box::new(Expr::Literal {
            value: Value::Number(21.0),
        }),
        operator: Operator {
            operator_type: OperatorType::Slash,
            src_line: 1,
            src_col: 4,
        },
        right: Box::new(Expr::Literal {
            value: Value::Number(7.0),
        }),
    });

    assert_eq!(result, expected);
    // Ensure all tokens were consumed
    assert!(parser.is_at_end());
}

#[test]
fn can_parse_grouping_expression() {
    let tokens = tokenize("((1 + 1))");

    let mut parser = Parser::new(tokens);
    let result = parser.expression();

    let expected = Ok(Expr::Grouping {
        inner: Box::new(Expr::Grouping {
            inner: Box::new(Expr::Binary {
                left: Box::new(Expr::Literal {
                    value: Value::Number(1.0),
                }),
                operator: Operator {
                    operator_type: OperatorType::Plus,
                    src_line: 1,
                    src_col: 5,
                },
                right: Box::new(Expr::Literal {
                    value: Value::Number(1.0),
                }),
            }),
        }),
    });

    assert_eq!(result, expected);
    // Ensure all tokens were consumed
    assert!(parser.is_at_end());
}

#[test]
fn error_on_unclosed_group_expression() {
    let tokens = tokenize("((1 + 1)");

    let mut parser = Parser::new(tokens);
    let result = parser.expression();

    let expected = Err(ParserError::MissingExpectedToken {
        token_type: TokenType::RightParen,
        message: "expected ')' after expression.".to_owned(),
    });

    assert_eq!(result, expected);
}

#[test]
fn can_parse_literal_expression() {
    let tokens = [tokenize("\"Hello, world!\""), tokenize("1")];

    let result = tokens
        .into_iter()
        .enumerate()
        .map(|(i, tokens)| {
            Parser::new(tokens)
                .expression()
                .unwrap_or_else(|_| panic!("failed to parse token at index {i}"))
        })
        .collect::<Vec<Expr>>();

    let expected = [
        Expr::Literal {
            value: Value::String("Hello, world!".to_owned()),
        },
        Expr::Literal {
            value: Value::Number(1.0),
        },
    ];

    assert_eq!(result, expected);
}

#[test]
fn can_parse_unary_expression() {
    let tokens = [tokenize("-1"), tokenize("!true"), tokenize("!(1 <= 2)")];

    let result = tokens
        .into_iter()
        .enumerate()
        .map(|(i, tokens)| {
            Parser::new(tokens)
                .expression()
                .unwrap_or_else(|_| panic!("failed to parse token at index {i}"))
        })
        .collect::<Vec<Expr>>();

    let expected = [
        Expr::Unary {
            operator: Operator {
                operator_type: OperatorType::Minus,
                src_line: 1,
                src_col: 1,
            },
            right: Box::new(Expr::Literal {
                value: Value::Number(1.0),
            }),
        },
        Expr::Unary {
            operator: Operator {
                operator_type: OperatorType::Bang,
                src_line: 1,
                src_col: 1,
            },
            right: Box::new(Expr::Literal {
                value: Value::Boolean(true),
            }),
        },
        Expr::Unary {
            operator: Operator {
                operator_type: OperatorType::Bang,
                src_line: 1,
                src_col: 1,
            },
            right: Box::new(Expr::Grouping {
                inner: Box::new(Expr::Binary {
                    left: Box::new(Expr::Literal {
                        value: Value::Number(1.0),
                    }),
                    operator: Operator {
                        operator_type: OperatorType::LessEqual,
                        src_line: 1,
                        src_col: 6,
                    },
                    right: Box::new(Expr::Literal {
                        value: Value::Number(2.0),
                    }),
                }),
            }),
        },
    ];

    assert_eq!(result, expected);
}

#[test]
fn can_parse_variable_expression() {
    let tokens = tokenize("a");

    let mut parser = Parser::new(tokens);
    let expression = parser.expression().unwrap();

    let expected = Expr::Variable {
        name: Token {
            token_type: TokenType::Identifier("a".to_owned()),
            line: 1,
            col: 1,
        },
    };

    assert_eq!(expression, expected);
}

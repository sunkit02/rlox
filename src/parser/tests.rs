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

#[test]
fn can_parse_if_statements_with_block_body() {
    let source = "if (condition) { print 1; print 2; }";
    let mut parser = Parser::new(tokenize(source));

    let stmts = parser.parse().unwrap();

    let expected = Stmt::If {
        condition: Expr::Variable {
            name: Token {
                token_type: TokenType::Identifier("condition".to_owned()),
                line: 1,
                col: 13,
            },
        },
        then_branch: Box::new(Stmt::Block(vec![
            Stmt::Print(Expr::Literal {
                value: Value::Number(1.0),
            }),
            Stmt::Print(Expr::Literal {
                value: Value::Number(2.0),
            }),
        ])),
        else_branch: None,
    };

    assert!(stmts.len() == 1);
    assert_eq!(stmts[0], expected);
}

#[test]
fn can_parse_if_statements_with_single_statement_body() {
    let source = "if (condition) print 1; if (condition) i = 2;";
    let mut parser = Parser::new(tokenize(source));

    let stmts = parser.parse().unwrap();

    let expected = vec![
        Stmt::If {
            condition: Expr::Variable {
                name: Token {
                    token_type: TokenType::Identifier("condition".to_owned()),
                    line: 1,
                    col: 13,
                },
            },
            then_branch: Box::new(Stmt::Print(Expr::Literal {
                value: Value::Number(1.0),
            })),
            else_branch: None,
        },
        Stmt::If {
            condition: Expr::Variable {
                name: Token {
                    token_type: TokenType::Identifier("condition".to_owned()),
                    line: 1,
                    col: 37,
                },
            },
            then_branch: Box::new(Stmt::Expression(Expr::Assign {
                name: Token {
                    token_type: TokenType::Identifier("i".to_owned()),
                    line: 1,
                    col: 40,
                },
                value: Box::new(Expr::Literal {
                    value: Value::Number(2.0),
                }),
            })),
            else_branch: None,
        },
    ];

    assert_eq!(stmts, expected);
}

#[test]
fn can_parse_nested_if_statements() {
    let source = "if (condition1) { if (condition2) { if (condition3) print 1; }}";
    let mut parser = Parser::new(tokenize(source));

    let stmts = parser.parse().unwrap();

    let expected = Stmt::If {
        condition: Expr::Variable {
            name: Token {
                token_type: TokenType::Identifier("condition1".to_owned()),
                line: 1,
                col: 14,
            },
        },
        then_branch: Box::new(Stmt::Block(vec![Stmt::If {
            condition: Expr::Variable {
                name: Token {
                    token_type: TokenType::Identifier("condition2".to_owned()),
                    line: 1,
                    col: 32,
                },
            },
            then_branch: Box::new(Stmt::Block(vec![Stmt::If {
                condition: Expr::Variable {
                    name: Token {
                        token_type: TokenType::Identifier("condition3".to_owned()),
                        line: 1,
                        col: 50,
                    },
                },
                then_branch: Box::new(Stmt::Print(Expr::Literal {
                    value: Value::Number(1.0),
                })),
                else_branch: None,
            }])),
            else_branch: None,
        }])),
        else_branch: None,
    };

    assert_eq!(stmts.len(), 1);
    assert_eq!(stmts[0], expected);
}

#[test]
fn can_parse_else_if_statements() {
    let source = r#"
    if (condition1) { 
        print 1; 
    } else if (condition2) {
        print 2;
    } else if (condition3) 
        print 3;
    else 
        print 4;"#;

    let mut parser = Parser::new(tokenize(source));

    let stmts = parser.parse().unwrap();

    let expected = Stmt::If {
        condition: Expr::Variable {
            name: Token {
                token_type: TokenType::Identifier("condition1".to_owned()),
                line: 2,
                col: 18,
            },
        },
        then_branch: Box::new(Stmt::Block(vec![Stmt::Print(Expr::Literal {
            value: Value::Number(1.0),
        })])),
        else_branch: Some(Box::new(Stmt::If {
            condition: Expr::Variable {
                name: Token {
                    token_type: TokenType::Identifier("condition2".to_owned()),
                    line: 4,
                    col: 25,
                },
            },
            then_branch: Box::new(Stmt::Block(vec![Stmt::Print(Expr::Literal {
                value: Value::Number(2.0),
            })])),
            else_branch: Some(Box::new(Stmt::If {
                condition: Expr::Variable {
                    name: Token {
                        token_type: TokenType::Identifier("condition3".to_owned()),
                        line: 6,
                        col: 25,
                    },
                },
                then_branch: Box::new(Stmt::Print(Expr::Literal {
                    value: Value::Number(3.0),
                })),
                else_branch: Some(Box::new(Stmt::Print(Expr::Literal {
                    value: Value::Number(4.0),
                }))),
            })),
        })),
    };

    assert_eq!(stmts.len(), 1);
    assert_eq!(stmts[0], expected);
}

#[test]
fn can_parse_while_loop_with_block_body() {
    let source = r#"while (true) { print 1; }"#;
    let mut parser = Parser::new(tokenize(source));

    let stmts = parser.parse().unwrap();

    let expected = Stmt::While {
        condition: Expr::Literal {
            value: Value::Boolean(true),
        },
        body: Box::new(Stmt::Block(vec![Stmt::Print(Expr::Literal {
            value: Value::Number(1.0),
        })])),
    };

    assert_eq!(stmts.len(), 1);
    assert_eq!(stmts[0], expected);
}

#[test]
fn can_parse_while_loop_with_single_statement_body() {
    let source = r#"while (true) print 1;"#;
    let mut parser = Parser::new(tokenize(source));

    let stmts = parser.parse().unwrap();

    let expected = Stmt::While {
        condition: Expr::Literal {
            value: Value::Boolean(true),
        },
        body: Box::new(Stmt::Print(Expr::Literal {
            value: Value::Number(1.0),
        })),
    };

    assert_eq!(stmts.len(), 1);
    assert_eq!(stmts[0], expected);
}

#[test]
fn can_parse_for_loop_with_block_body() {
    let source = r#"for (var i = 0; i < 10; i = i + 1) { print i; }"#;
    let mut parser = Parser::new(tokenize(source));

    let stmts = parser.parse().unwrap();

    let expected = Stmt::Block(vec![
        Stmt::Var {
            name: Token {
                token_type: TokenType::Identifier("i".to_owned()),
                line: 1,
                col: 10,
            },
            initializer: Some(Expr::Literal {
                value: Value::Number(0.0),
            }),
        },
        Stmt::While {
            condition: Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: Token {
                        token_type: TokenType::Identifier("i".to_owned()),
                        line: 1,
                        col: 17,
                    },
                }),
                operator: Operator {
                    operator_type: OperatorType::Less,
                    src_line: 1,
                    src_col: 19,
                },
                right: Box::new(Expr::Literal {
                    value: Value::Number(10.0),
                }),
            },
            body: Box::new(Stmt::Block(vec![
                Stmt::Print(Expr::Variable {
                    name: Token {
                        token_type: TokenType::Identifier("i".to_owned()),
                        line: 1,
                        col: 44,
                    },
                }),
                Stmt::Expression(Expr::Assign {
                    name: Token {
                        token_type: TokenType::Identifier("i".to_owned()),
                        line: 1,
                        col: 25,
                    },
                    value: Box::new(Expr::Binary {
                        left: Box::new(Expr::Variable {
                            name: Token {
                                token_type: TokenType::Identifier("i".to_owned()),
                                line: 1,
                                col: 29,
                            },
                        }),
                        operator: Operator {
                            operator_type: OperatorType::Plus,
                            src_line: 1,
                            src_col: 31,
                        },
                        right: Box::new(Expr::Literal {
                            value: Value::Number(1.0),
                        }),
                    }),
                }),
            ])),
        },
    ]);

    assert_eq!(stmts.len(), 1);
    assert_eq!(stmts[0], expected);
}

#[test]
fn can_parse_for_loop_with_single_statement_body() {
    let source = r#"for (var i = 0; i < 10; i = i + 1) print i;"#;
    let mut parser = Parser::new(tokenize(source));

    let stmts = parser.parse().unwrap();

    let expected = Stmt::Block(vec![
        Stmt::Var {
            name: Token {
                token_type: TokenType::Identifier("i".to_owned()),
                line: 1,
                col: 10,
            },
            initializer: Some(Expr::Literal {
                value: Value::Number(0.0),
            }),
        },
        Stmt::While {
            condition: Expr::Binary {
                left: Box::new(Expr::Variable {
                    name: Token {
                        token_type: TokenType::Identifier("i".to_owned()),
                        line: 1,
                        col: 17,
                    },
                }),
                operator: Operator {
                    operator_type: OperatorType::Less,
                    src_line: 1,
                    src_col: 19,
                },
                right: Box::new(Expr::Literal {
                    value: Value::Number(10.0),
                }),
            },
            body: Box::new(Stmt::Block(vec![
                Stmt::Print(Expr::Variable {
                    name: Token {
                        token_type: TokenType::Identifier("i".to_owned()),
                        line: 1,
                        col: 42,
                    },
                }),
                Stmt::Expression(Expr::Assign {
                    name: Token {
                        token_type: TokenType::Identifier("i".to_owned()),
                        line: 1,
                        col: 25,
                    },
                    value: Box::new(Expr::Binary {
                        left: Box::new(Expr::Variable {
                            name: Token {
                                token_type: TokenType::Identifier("i".to_owned()),
                                line: 1,
                                col: 29,
                            },
                        }),
                        operator: Operator {
                            operator_type: OperatorType::Plus,
                            src_line: 1,
                            src_col: 31,
                        },
                        right: Box::new(Expr::Literal {
                            value: Value::Number(1.0),
                        }),
                    }),
                }),
            ])),
        },
    ]);

    assert_eq!(stmts.len(), 1);
    assert_eq!(stmts[0], expected);
}

#[test]
fn can_parse_for_loop_with_empty_clauses() {
    let source = r#"for (;;) print i;"#;
    let mut parser = Parser::new(tokenize(source));

    let stmts = parser.parse().unwrap();

    // There should be no extra block around the while loop if there is no
    // initializer in the for loop.
    let expected = Stmt::While {
        condition: Expr::Literal {
            value: Value::Boolean(true),
        },
        body: Box::new(Stmt::Print(Expr::Variable {
            name: Token {
                token_type: TokenType::Identifier("i".to_owned()),
                line: 1,
                col: 16,
            },
        })),
    };

    assert_eq!(stmts.len(), 1);
    assert_eq!(stmts[0], expected);
}

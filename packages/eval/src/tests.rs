use ast::{AssignmentExpr, Closure, If, Location, Null, Return, Str, Symbol, While};
use scope::HashScope;

use super::*;

#[test]
fn eval_primitive() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);
    let result = eval(
        Expr::Literal(Literal::Int(Int {
            value: 1,
            location: Default::default(),
        })),
        &ctx,
    );
    assert_eq!(
        result,
        Ok(Literal::Int(Int {
            value: 1,
            location: Default::default()
        }))
    );

    let result = eval(
        Expr::Literal(Literal::Bool(Boolean {
            value: true,
            location: Default::default(),
        })),
        &ctx,
    );
    assert_eq!(
        result,
        Ok(Literal::Bool(Boolean {
            value: true,
            location: Default::default()
        }))
    );

    let result = eval(
        Expr::Literal(Literal::String(Str {
            value: String::from("test"),
            location: Default::default(),
        })),
        &ctx,
    );
    assert_eq!(
        result,
        Ok(Literal::String(Str {
            value: String::from("test"),
            location: Default::default()
        }))
    );

    let result = eval(
        Expr::Literal(Literal::Float(Float {
            value: 1.5,
            location: Default::default(),
        })),
        &ctx,
    );
    assert_eq!(
        result,
        Ok(Literal::Float(Float {
            value: 1.5,
            location: Default::default()
        }))
    );

    eval(
        Expr::Assignment(AssignmentExpr {
            symbol: String::from("name"),
            value: Box::new(Expr::Literal(Literal::Int(Int {
                value: 4,
                location: Default::default(),
            }))),
            location: Location::default(),
        }),
        &ctx,
    )
    .unwrap();
    let symbol = Expr::Symbol(Symbol {
        value: String::from("name"),
        location: Location::default(),
    });
    let found_value = eval(symbol, &ctx);
    assert_eq!(
        found_value,
        Ok(Literal::Int(Int {
            value: 4,
            location: Default::default()
        }))
    )
}
#[test]
fn eval_add_operation() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);
    let op = Expr::BinaryExpr(Box::new(BinaryExpr {
        left: Expr::BinaryExpr(Box::new(BinaryExpr::new(
            Expr::Literal(Literal::Int(Int {
                value: 2,
                location: Default::default(),
            })),
            Expr::Literal(Literal::Int(Int {
                value: 8,
                location: Default::default(),
            })),
            BinaryOperator::Add,
        ))),
        right: Expr::BinaryExpr(Box::new(BinaryExpr::new(
            Expr::Literal(Literal::Float(Float {
                value: 4.5,
                location: Default::default(),
            })),
            Expr::Literal(Literal::Int(Int {
                value: 5,
                location: Default::default(),
            })),
            BinaryOperator::Add,
        ))),
        operator: BinaryOperator::Add,
        location: Location::default(),
    }));
    let result = eval(op, &ctx);
    assert_eq!(
        result,
        Ok(Literal::Float(Float {
            value: 19.5,
            location: Default::default()
        }))
    );
}
#[test]
#[should_panic]
fn try_operate_string() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);

    let op = Expr::BinaryExpr(Box::new(BinaryExpr::new(
        Expr::Literal(Literal::String(Str {
            value: String::from("Gab"),
            location: Default::default(),
        })),
        Expr::Literal(Literal::String(Str {
            value: String::from("riel"),
            location: Default::default(),
        })),
        BinaryOperator::Add,
    )));
    eval(op, &ctx).unwrap();
}
#[test]
fn eval_sub_operation() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);
    let op = Expr::BinaryExpr(Box::new(BinaryExpr {
        left: Expr::BinaryExpr(Box::new(BinaryExpr::new(
            Expr::Literal(Literal::Int(Int {
                value: 8,
                location: Default::default(),
            })),
            Expr::Literal(Literal::Int(Int {
                value: 6,
                location: Default::default(),
            })),
            BinaryOperator::Sub,
        ))),
        right: Expr::BinaryExpr(Box::new(BinaryExpr::new(
            Expr::Literal(Literal::Float(Float {
                value: 4.5,
                location: Default::default(),
            })),
            Expr::Literal(Literal::Float(Float {
                value: 3.5,
                location: Default::default(),
            })),
            BinaryOperator::Sub,
        ))),
        operator: BinaryOperator::Add,
        location: Location::default(),
    }));
    let result = eval(op, &ctx);
    assert_eq!(
        result,
        Ok(Literal::Float(Float {
            value: 3.0,
            location: Default::default()
        }))
    );
}
#[test]
fn eval_multiplication() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);

    let op = Expr::BinaryExpr(Box::new(BinaryExpr {
        left: Expr::BinaryExpr(Box::new(BinaryExpr::new(
            Expr::Literal(Literal::Int(Int {
                value: 2,
                location: Default::default(),
            })),
            Expr::Literal(Literal::Int(Int {
                value: 8,
                location: Default::default(),
            })),
            BinaryOperator::Mul,
        ))),
        right: Expr::BinaryExpr(Box::new(BinaryExpr::new(
            Expr::Literal(Literal::Float(Float {
                value: 4.5,
                location: Default::default(),
            })),
            Expr::Literal(Literal::Int(Int {
                value: 5,
                location: Default::default(),
            })),
            BinaryOperator::Mul,
        ))),
        operator: BinaryOperator::Add,
        location: Location::default(),
    }));
    let result = eval(op, &ctx);
    assert_eq!(
        result,
        Ok(Literal::Float(Float {
            value: 38.5,
            location: Default::default()
        }))
    );
}
#[test]
fn eval_division() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);

    ctx.scope.set(
        "age",
        Literal::Int(Int {
            value: 10,
            location: Default::default(),
        }),
    );

    let op = Expr::BinaryExpr(Box::new(BinaryExpr {
        left: Expr::BinaryExpr(Box::new(BinaryExpr::new(
            Expr::Symbol(Symbol {
                value: String::from("age"),
                location: Location::default(),
            }),
            Expr::Literal(Literal::Int(Int {
                value: 2,
                location: Default::default(),
            })),
            BinaryOperator::Div,
        ))),
        right: Expr::BinaryExpr(Box::new(BinaryExpr::new(
            Expr::Literal(Literal::Int(Int {
                value: 5,
                location: Default::default(),
            })),
            Expr::Literal(Literal::Float(Float {
                value: 0.5,
                location: Default::default(),
            })),
            BinaryOperator::Div,
        ))),
        operator: BinaryOperator::Add,
        location: Location::default(),
    }));
    let result = eval(op, &ctx);
    assert_eq!(
        result,
        Ok(Literal::Float(Float {
            value: 15.0,
            location: Default::default()
        }))
    );
}
#[test]
fn eval_gt() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);

    let op = Expr::BinaryExpr(Box::new(BinaryExpr::new(
        Expr::Literal(Literal::Int(Int {
            value: 8,
            location: Default::default(),
        })),
        Expr::Literal(Literal::Int(Int {
            value: 4,
            location: Default::default(),
        })),
        BinaryOperator::Gt,
    )));
    let result = eval(op, &ctx);
    assert_eq!(
        result,
        Ok(Literal::Bool(Boolean {
            value: true,
            location: Default::default()
        }))
    );
}
#[test]
fn truthy_or_falsy() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);

    assert_eq!(
        is_truthy(
            Expr::Literal(Literal::Null(Null {
                location: Default::default()
            })),
            &ctx,
        ),
        Ok(false)
    );
    assert_eq!(
        is_truthy(
            Expr::Literal(Literal::String(Str {
                value: String::from(""),
                location: Default::default()
            })),
            &ctx,
        ),
        Ok(false)
    );
    assert_eq!(
        is_truthy(
            Expr::Literal(Literal::String(Str {
                value: String::from("Test"),
                location: Default::default()
            })),
            &ctx,
        ),
        Ok(true)
    );
    assert_eq!(
        is_truthy(
            Expr::Literal(Literal::Bool(Boolean {
                value: true,
                location: Default::default()
            })),
            &ctx,
        ),
        Ok(true)
    );
    assert_eq!(
        is_truthy(
            Expr::Literal(Literal::Bool(Boolean {
                value: false,
                location: Default::default()
            })),
            &ctx,
        ),
        Ok(false)
    );
    assert_eq!(
        is_truthy(
            Expr::Literal(Literal::Int(Int {
                value: 0,
                location: Default::default()
            })),
            &ctx,
        ),
        Ok(false)
    );
    assert_eq!(
        is_truthy(
            Expr::Literal(Literal::Int(Int {
                value: 1,
                location: Default::default()
            })),
            &ctx,
        ),
        Ok(true)
    );
    assert_eq!(
        is_truthy(
            Expr::Literal(Literal::Float(Float {
                value: 1.1,
                location: Default::default()
            })),
            &ctx,
        ),
        Ok(true)
    );
    assert_eq!(
        is_truthy(
            Expr::Literal(Literal::Float(Float {
                value: 0.0,
                location: Default::default()
            })),
            &ctx,
        ),
        Ok(false)
    );
    assert_eq!(
        is_truthy(
            Expr::BinaryExpr(Box::new(BinaryExpr::new(
                Expr::Literal(Literal::Int(Int {
                    value: 4,
                    location: Default::default()
                })),
                Expr::Literal(Literal::Int(Int {
                    value: 7,
                    location: Default::default()
                })),
                BinaryOperator::Add
            ))),
            &ctx,
        ),
        Ok(true)
    );
    assert_eq!(
        is_truthy(
            Expr::BinaryExpr(Box::new(BinaryExpr::new(
                Expr::Literal(Literal::Int(Int {
                    value: 4,
                    location: Default::default()
                })),
                Expr::Literal(Literal::Int(Int {
                    value: 4,
                    location: Default::default()
                })),
                BinaryOperator::Sub
            ))),
            &ctx
        ),
        Ok(false)
    );
}
#[test]
fn logical_operations() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);
    let op = Expr::BinaryExpr(Box::new(BinaryExpr::new(
        Expr::Literal(Literal::Bool(Boolean {
            value: true,
            location: Default::default(),
        })),
        Expr::Literal(Literal::Bool(Boolean {
            value: false,
            location: Default::default(),
        })),
        BinaryOperator::Or,
    )));
    assert_eq!(
        eval(op, &ctx),
        Ok(Literal::Bool(Boolean {
            value: true,
            location: Default::default()
        }))
    );

    let op = Expr::BinaryExpr(Box::new(BinaryExpr::new(
        Expr::Literal(Literal::Bool(Boolean {
            value: true,
            location: Default::default(),
        })),
        Expr::Literal(Literal::Bool(Boolean {
            value: false,
            location: Default::default(),
        })),
        BinaryOperator::And,
    )));
    assert_eq!(
        eval(op, &ctx),
        Ok(Literal::Bool(Boolean {
            value: false,
            location: Default::default()
        }))
    );

    let op = Expr::BinaryExpr(Box::new(BinaryExpr::new(
        Expr::Literal(Literal::Bool(Boolean {
            value: true,
            location: Default::default(),
        })),
        Expr::Literal(Literal::Bool(Boolean {
            value: true,
            location: Default::default(),
        })),
        BinaryOperator::And,
    )));
    assert_eq!(
        eval(op, &ctx),
        Ok(Literal::Bool(Boolean {
            value: true,
            location: Default::default()
        }))
    );

    let op = Expr::BinaryExpr(Box::new(BinaryExpr::new(
        Expr::Literal(Literal::Bool(Boolean {
            value: false,
            location: Default::default(),
        })),
        Expr::Literal(Literal::Bool(Boolean {
            value: false,
            location: Default::default(),
        })),
        BinaryOperator::Or,
    )));
    assert_eq!(
        eval(op, &ctx),
        Ok(Literal::Bool(Boolean {
            value: false,
            location: Default::default()
        }))
    );
}
#[test]
fn test_eval_call() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);
    ctx.scope.set(
        "greet",
        Literal::Closure(ast::Closure {
            params: vec![String::from("name")],
            body: vec![Stmt::Return(Return {
                value: Expr::Symbol(Symbol {
                    value: String::from("name"),
                    location: Location::default(),
                }),
                location: Location::default(),
            })],
            location: Location::default(),
        }),
    );
    let call = Expr::Call(Call {
        symbol: String::from("greet"),
        args: vec![Expr::Literal(Literal::String(Str {
            value: String::from("John"),
            location: Default::default(),
        }))],
        location: Location::default(),
    });
    let result = eval(call, &ctx);
    assert_eq!(
        result,
        Ok(Literal::String(Str {
            value: String::from("John"),
            location: Default::default()
        }))
    );
}
#[test]
fn test_if_else() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);
    let is_adult_fn = Closure {
        params: vec![String::from("age")],
        body: vec![Stmt::If(If {
            cond: Expr::BinaryExpr(Box::new(BinaryExpr::new(
                Expr::Symbol(Symbol {
                    value: String::from("age"),
                    location: Location::default(),
                }),
                Expr::Literal(Literal::Int(Int {
                    value: 18,
                    location: Default::default(),
                })),
                BinaryOperator::Ge,
            ))),
            body: vec![Stmt::Return(Return {
                value: Expr::Literal(Literal::Bool(Boolean {
                    value: true,
                    location: Default::default(),
                })),
                location: Location::default(),
            })],
            else_block: Some(vec![Stmt::Return(Return {
                value: Expr::Literal(Literal::Bool(Boolean {
                    value: false,
                    location: Default::default(),
                })),
                location: Location::default(),
            })]),
            location: Location::default(),
        })],
        location: Location::default(),
    };
    // Rust equivalent to this function:
    // fn is_adult(age: i64) -> bool {
    //  if age >= 18 {
    //      true
    //  } else {
    //      false
    //  }
    // }
    ctx.scope.set("is_adult", Literal::Closure(is_adult_fn));
    let call = Expr::Call(Call {
        symbol: String::from("is_adult"),
        args: vec![Expr::Literal(Literal::Int(Int {
            value: 18,
            location: Default::default(),
        }))],
        location: Location::default(),
    });
    let result = eval(call, &ctx);
    assert_eq!(
        result,
        Ok(Literal::Bool(Boolean {
            value: true,
            location: Default::default()
        }))
    );

    let call = Expr::Call(Call {
        symbol: String::from("is_adult"),
        args: vec![Expr::Literal(Literal::Int(Int {
            value: 17,
            location: Default::default(),
        }))],
        location: Location::default(),
    });
    let result = eval(call, &ctx);
    assert_eq!(
        result,
        Ok(Literal::Bool(Boolean {
            value: false,
            location: Default::default()
        }))
    );
}
#[test]
fn test_while_loop() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);
    ctx.scope.set(
        "count",
        Literal::Int(Int {
            value: 0,
            location: Default::default(),
        }),
    );
    let program: Program = vec![Stmt::While(While {
        cond: Expr::BinaryExpr(Box::new(BinaryExpr::new(
            Expr::Symbol(Symbol {
                value: String::from("count"),
                location: Location::default(),
            }),
            Expr::Literal(Literal::Int(Int {
                value: 10,
                location: Default::default(),
            })),
            BinaryOperator::Lt,
        ))),
        body: vec![Stmt::Expr(Expr::Assignment(AssignmentExpr {
            symbol: String::from("count"),
            value: Box::new(Expr::BinaryExpr(Box::new(BinaryExpr::new(
                Expr::Symbol(Symbol {
                    value: String::from("count"),
                    location: Location::default(),
                }),
                Expr::Literal(Literal::Int(Int {
                    value: 1,
                    location: Default::default(),
                })),
                BinaryOperator::Add,
            )))),
            location: Location::default(),
        }))],
        location: Location::default(),
    })];
    // Rust equivalent
    // while count < 10 {
    //  count = count + 1;
    // }
    eval_program(program, &ctx).unwrap();
    let final_count = ctx.scope.get("count");
    assert_eq!(
        final_count,
        Literal::Int(Int {
            value: 10,
            location: Default::default()
        })
    );
}
#[test]
fn test_unary_op() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);
    assert_eq!(
        eval(
            Expr::UnaryExpr(Box::new(UnaryExpr {
                operator: ast::UnaryOperator::Not,
                operand: Expr::Literal(Literal::Bool(Boolean {
                    value: true,
                    location: Default::default()
                })),
                location: Location::default(),
            })),
            &ctx,
        ),
        Ok(Literal::Bool(Boolean {
            value: false,
            location: Default::default()
        }))
    );
    assert_eq!(
        eval(
            Expr::UnaryExpr(Box::new(UnaryExpr {
                operator: ast::UnaryOperator::Not,
                operand: Expr::Literal(Literal::Bool(Boolean {
                    value: false,
                    location: Default::default()
                })),
                location: Location::default(),
            })),
            &ctx,
        ),
        Ok(Literal::Bool(Boolean {
            value: true,
            location: Default::default()
        }))
    );
}

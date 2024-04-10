use ast::{AssignmentExpr, Closure, If, Location, While};
use scope::HashScope;

use super::*;

#[test]
fn eval_primitive() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);
    let result = eval(Expr::Literal(Literal::Int(1)), &ctx);
    assert_eq!(result, Ok(Literal::Int(1)));

    let result = eval(Expr::Literal(Literal::Bool(true)), &ctx);
    assert_eq!(result, Ok(Literal::Bool(true)));

    let result = eval(Expr::Literal(Literal::String(String::from("test"))), &ctx);
    assert_eq!(result, Ok(Literal::String(String::from("test"))));

    let result = eval(Expr::Literal(Literal::Float(1.5)), &ctx);
    assert_eq!(result, Ok(Literal::Float(1.5)));

    eval(
        Expr::Assignment(AssignmentExpr {
            symbol: String::from("name"),
            value: Box::new(Expr::Literal(Literal::Int(4))),
            location: Location::default(),
        }),
        &ctx,
    )
    .unwrap();
    let symbol = Expr::Symbol(String::from("name"));
    let found_value = eval(symbol, &ctx);
    assert_eq!(found_value, Ok(Literal::Int(4)))
}
#[test]
fn eval_add_operation() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);
    let op = Expr::BinaryExpr(Box::new(BinaryExpr {
        left: Expr::BinaryExpr(Box::new(BinaryExpr::new(
            Expr::Literal(Literal::Int(2)),
            Expr::Literal(Literal::Int(8)),
            BinaryOperator::Add,
        ))),
        right: Expr::BinaryExpr(Box::new(BinaryExpr::new(
            Expr::Literal(Literal::Float(4.5)),
            Expr::Literal(Literal::Int(5)),
            BinaryOperator::Add,
        ))),
        operator: BinaryOperator::Add,
        location: Location::default(),
    }));
    let result = eval(op, &ctx);
    assert_eq!(result, Ok(Literal::Float(19.5)));
}
#[test]
#[should_panic]
fn try_operate_string() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);

    let op = Expr::BinaryExpr(Box::new(BinaryExpr::new(
        Expr::Literal(Literal::String(String::from("Gab"))),
        Expr::Literal(Literal::String(String::from("riel"))),
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
            Expr::Literal(Literal::Int(8)),
            Expr::Literal(Literal::Int(6)),
            BinaryOperator::Sub,
        ))),
        right: Expr::BinaryExpr(Box::new(BinaryExpr::new(
            Expr::Literal(Literal::Float(4.5)),
            Expr::Literal(Literal::Float(3.5)),
            BinaryOperator::Sub,
        ))),
        operator: BinaryOperator::Add,
        location: Location::default(),
    }));
    let result = eval(op, &ctx);
    assert_eq!(result, Ok(Literal::Float(3.0)));
}
#[test]
fn eval_multiplication() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);

    let op = Expr::BinaryExpr(Box::new(BinaryExpr {
        left: Expr::BinaryExpr(Box::new(BinaryExpr::new(
            Expr::Literal(Literal::Int(2)),
            Expr::Literal(Literal::Int(8)),
            BinaryOperator::Mul,
        ))),
        right: Expr::BinaryExpr(Box::new(BinaryExpr::new(
            Expr::Literal(Literal::Float(4.5)),
            Expr::Literal(Literal::Int(5)),
            BinaryOperator::Mul,
        ))),
        operator: BinaryOperator::Add,
        location: Location::default(),
    }));
    let result = eval(op, &ctx);
    assert_eq!(result, Ok(Literal::Float(38.5)));
}
#[test]
fn eval_division() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);

    ctx.scope.set("age", Literal::Int(10));

    let op = Expr::BinaryExpr(Box::new(BinaryExpr {
        left: Expr::BinaryExpr(Box::new(BinaryExpr::new(
            Expr::Symbol(String::from("age")),
            Expr::Literal(Literal::Int(2)),
            BinaryOperator::Div,
        ))),
        right: Expr::BinaryExpr(Box::new(BinaryExpr::new(
            Expr::Literal(Literal::Int(5)),
            Expr::Literal(Literal::Float(0.5)),
            BinaryOperator::Div,
        ))),
        operator: BinaryOperator::Add,
        location: Location::default(),
    }));
    let result = eval(op, &ctx);
    assert_eq!(result, Ok(Literal::Float(15.0)));
}
#[test]
fn eval_gt() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);

    let op = Expr::BinaryExpr(Box::new(BinaryExpr::new(
        Expr::Literal(Literal::Int(8)),
        Expr::Literal(Literal::Int(4)),
        BinaryOperator::Gt,
    )));
    let result = eval(op, &ctx);
    assert_eq!(result, Ok(Literal::Bool(true)));
}
#[test]
fn truthy_or_falsy() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);

    assert_eq!(is_truthy(Expr::Literal(Literal::Null), &ctx), Ok(false));
    assert_eq!(
        is_truthy(Expr::Literal(Literal::String(String::from(""))), &ctx),
        Ok(false)
    );
    assert_eq!(
        is_truthy(Expr::Literal(Literal::String(String::from("Test"))), &ctx),
        Ok(true)
    );
    assert_eq!(
        is_truthy(Expr::Literal(Literal::Bool(true)), &ctx),
        Ok(true)
    );
    assert_eq!(
        is_truthy(Expr::Literal(Literal::Bool(false)), &ctx),
        Ok(false)
    );
    assert_eq!(is_truthy(Expr::Literal(Literal::Int(0)), &ctx), Ok(false));
    assert_eq!(is_truthy(Expr::Literal(Literal::Int(1)), &ctx), Ok(true));
    assert_eq!(
        is_truthy(Expr::Literal(Literal::Float(1.1)), &ctx),
        Ok(true)
    );
    assert_eq!(
        is_truthy(Expr::Literal(Literal::Float(0.0)), &ctx),
        Ok(false)
    );
    assert_eq!(
        is_truthy(
            Expr::BinaryExpr(Box::new(BinaryExpr::new(
                Expr::Literal(Literal::Int(4)),
                Expr::Literal(Literal::Int(7)),
                BinaryOperator::Add
            ))),
            &ctx
        ),
        Ok(true)
    );
    assert_eq!(
        is_truthy(
            Expr::BinaryExpr(Box::new(BinaryExpr::new(
                Expr::Literal(Literal::Int(4)),
                Expr::Literal(Literal::Int(4)),
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
        Expr::Literal(Literal::Bool(true)),
        Expr::Literal(Literal::Bool(false)),
        BinaryOperator::Or,
    )));
    assert_eq!(eval(op, &ctx), Ok(Literal::Bool(true)));

    let op = Expr::BinaryExpr(Box::new(BinaryExpr::new(
        Expr::Literal(Literal::Bool(true)),
        Expr::Literal(Literal::Bool(false)),
        BinaryOperator::And,
    )));
    assert_eq!(eval(op, &ctx), Ok(Literal::Bool(false)));

    let op = Expr::BinaryExpr(Box::new(BinaryExpr::new(
        Expr::Literal(Literal::Bool(true)),
        Expr::Literal(Literal::Bool(true)),
        BinaryOperator::And,
    )));
    assert_eq!(eval(op, &ctx), Ok(Literal::Bool(true)));

    let op = Expr::BinaryExpr(Box::new(BinaryExpr::new(
        Expr::Literal(Literal::Bool(false)),
        Expr::Literal(Literal::Bool(false)),
        BinaryOperator::Or,
    )));
    assert_eq!(eval(op, &ctx), Ok(Literal::Bool(false)));
}
#[test]
fn test_eval_call() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);
    ctx.scope.set(
        "greet",
        Literal::Closure(ast::Closure {
            params: vec![String::from("name")],
            body: vec![Instruction::Stmt(Stmt::Return(Expr::Symbol(String::from(
                "name",
            ))))],
            location: Location::default(),
        }),
    );
    let call = Expr::Call(Call {
        symbol: String::from("greet"),
        args: vec![Expr::Literal(Literal::String(String::from("John")))],
        location: Location::default(),
    });
    let result = eval(call, &ctx);
    assert_eq!(result, Ok(Literal::String(String::from("John"))));
}
#[test]
fn test_if_else() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);
    let is_adult_fn = Closure {
        params: vec![String::from("age")],
        body: vec![Instruction::Stmt(Stmt::If(If {
            cond: Expr::BinaryExpr(Box::new(BinaryExpr::new(
                Expr::Symbol(String::from("age")),
                Expr::Literal(Literal::Int(18)),
                BinaryOperator::Ge,
            ))),
            body: vec![Instruction::Stmt(Stmt::Return(Expr::Literal(
                Literal::Bool(true),
            )))],
            else_block: Some(vec![Instruction::Stmt(Stmt::Return(Expr::Literal(
                Literal::Bool(false),
            )))]),
            location: Location::default(),
        }))],
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
        args: vec![Expr::Literal(Literal::Int(18))],
        location: Location::default(),
    });
    let result = eval(call, &ctx);
    assert_eq!(result, Ok(Literal::Bool(true)));

    let call = Expr::Call(Call {
        symbol: String::from("is_adult"),
        args: vec![Expr::Literal(Literal::Int(17))],
        location: Location::default(),
    });
    let result = eval(call, &ctx);
    assert_eq!(result, Ok(Literal::Bool(false)));
}
#[test]
fn test_while_loop() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);
    ctx.scope.set("count", Literal::Int(0));
    let program: Program = vec![Instruction::Stmt(Stmt::While(While {
        cond: Expr::BinaryExpr(Box::new(BinaryExpr::new(
            Expr::Symbol(String::from("count")),
            Expr::Literal(Literal::Int(10)),
            BinaryOperator::Lt,
        ))),
        body: vec![Instruction::Expr(Expr::Assignment(AssignmentExpr {
            symbol: String::from("count"),
            value: Box::new(Expr::BinaryExpr(Box::new(BinaryExpr::new(
                Expr::Symbol(String::from("count")),
                Expr::Literal(Literal::Int(1)),
                BinaryOperator::Add,
            )))),
            location: Location::default(),
        }))],
        location: Location::default(),
    }))];
    // Rust equivalent
    // while count < 10 {
    //  count = count + 1;
    // }
    eval_program(program, &ctx).unwrap();
    let final_count = ctx.scope.get("count");
    assert_eq!(final_count, Literal::Int(10));
}
#[test]
fn test_unary_op() {
    let scope = HashScope::default();
    let ctx = Context::new(scope);
    assert_eq!(
        eval(
            Expr::UnaryExpr(Box::new(UnaryExpr {
                operator: ast::UnaryOperator::Not,
                operand: Expr::Literal(Literal::Bool(true)),
                location: Location::default(),
            })),
            &ctx
        ),
        Ok(Literal::Bool(false))
    );
    assert_eq!(
        eval(
            Expr::UnaryExpr(Box::new(UnaryExpr {
                operator: ast::UnaryOperator::Not,
                operand: Expr::Literal(Literal::Bool(false)),
                location: Location::default(),
            })),
            &ctx
        ),
        Ok(Literal::Bool(true))
    );
}

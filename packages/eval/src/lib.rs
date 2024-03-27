pub mod scope;

use ast::{BinaryOp, BinaryOpType, Call, Expr, Instruction, Literal, Program, Stmt};
use scope::Scope;

macro_rules! define_aritmetic_operation {
    ($operator:tt, $op:expr, $scope:expr) => {
        match ($op.left, $op.right) {
            (Expr::Literal(left), Expr::Literal(right)) => match (left, right) {
                (Literal::Int(left), Literal::Int(right)) => Literal::Int(left $operator right),
                (Literal::Float(left), Literal::Int(right)) => Literal::Float(left $operator (right as f64)),
                (Literal::Int(left), Literal::Float(right)) => Literal::Float((left as f64) $operator right),
                (Literal::Float(left), Literal::Float(right)) => Literal::Float(left $operator right),
                (_, _) => panic!("Unsuported operation"),
            },
            (left, right) => eval_binary_op(
                BinaryOp::new(
                    Expr::Literal(eval(left, $scope)),
                    Expr::Literal(eval(right, $scope)),
                    $op.op_type,
                ),
                $scope,
            ),
        }
    };
}

macro_rules! define_boolean_operation {
    ($operator:tt, $op:expr, $scope:expr) => {
        match ($op.left, $op.right) {
            (Expr::Literal(left), Expr::Literal(right)) => match (left, right) {
                (Literal::Int(left), Literal::Int(right)) => Literal::Bool(left $operator right),
                (Literal::Float(left), Literal::Int(right)) => Literal::Bool(left $operator (right as f64)),
                (Literal::Int(left), Literal::Float(right)) => Literal::Bool((left as f64) $operator right),
                (Literal::Float(left), Literal::Float(right)) => Literal::Bool(left $operator right),
                (_, _) => panic!("Unsuported operation"),
            },
            (left, right) => eval_binary_op(
                BinaryOp::new(
                    Expr::Literal(eval(left, $scope)),
                    Expr::Literal(eval(right, $scope)),
                    $op.op_type,
                ),
                $scope,
            ),
        }
    };
}

fn is_truthy<T: Scope + Clone>(expr: Expr, scope: &T) -> bool {
    match expr {
        Expr::Literal(value) => match value {
            Literal::Closure(_) => true,
            Literal::Int(num) => num != 0,
            Literal::Float(num) => num != 0.0,
            Literal::String(string) => !string.is_empty(),
            Literal::Bool(bool) => bool,
            Literal::Null => false,
            Literal::Void => false,
        },
        expr => is_truthy(Expr::Literal(eval(expr, scope)), scope),
    }
}

fn eval_binary_op<T: Scope + Clone>(op: BinaryOp, scope: &T) -> Literal {
    match op.op_type {
        BinaryOpType::Add => define_aritmetic_operation!(+, op, scope),
        BinaryOpType::Sub => define_aritmetic_operation!(-, op, scope),
        BinaryOpType::Mul => define_aritmetic_operation!(*, op, scope),
        BinaryOpType::Div => define_aritmetic_operation!(/, op, scope),
        BinaryOpType::Gt => define_boolean_operation!(>, op, scope),
        BinaryOpType::Eq => define_boolean_operation!(==, op, scope),
        BinaryOpType::Ge => define_boolean_operation!(>=, op, scope),
        BinaryOpType::Lt => define_boolean_operation!(<, op, scope),
        BinaryOpType::Le => define_boolean_operation!(<=, op, scope),
        BinaryOpType::And => Literal::Bool(is_truthy(op.left, scope) && is_truthy(op.right, scope)),
        BinaryOpType::Or => Literal::Bool(is_truthy(op.left, scope) || is_truthy(op.right, scope)),
    }
}

pub fn eval_program<T: Scope + Clone>(program: Program, scope: &T) -> Literal {
    for instruction in program {
        match instruction {
            Instruction::Stmt(stmt) => match stmt {
                Stmt::Return(val) => {
                    return eval(val, scope);
                }
                Stmt::If(if_stmt) => {
                    if is_truthy(if_stmt.cond, scope) {
                        let block_result = eval_program(if_stmt.body, scope);
                        match block_result {
                            Literal::Void => (),
                            val => return val,
                        }
                    } else if let Some(else_block) = if_stmt.else_block {
                        let block_result = eval_program(else_block, scope);
                        match block_result {
                            Literal::Null => (),
                            val => return val,
                        }
                    }
                }
                Stmt::While(while_stmt) => {
                    while is_truthy(while_stmt.clone().cond, scope) {
                        let block_result = eval_program(while_stmt.clone().body, scope);
                        match block_result {
                            Literal::Void => (),
                            val => return val,
                        }
                    }
                }
                Stmt::Print(expr) => {
                    let val = eval(expr, scope);
                    match val {
                        Literal::Closure(_) => println!("[closure]"),
                        Literal::Int(num) => println!("{num}"),
                        Literal::Float(num) => println!("{num}"),
                        Literal::String(string) => println!("{string}"),
                        Literal::Bool(bool) => {
                            if bool {
                                println!("True")
                            } else {
                                println!("False")
                            }
                        }
                        Literal::Null => println!("null"),
                        Literal::Void => (),
                    }
                }
            },
            Instruction::Expr(expr) => {
                eval(expr, scope);
            }
        }
    }
    Literal::Void
}

fn eval_call<T: Scope + Clone>(call: Call, scope: &T) -> Literal {
    let found = scope.get(&call.symbol);
    if let Literal::Closure(closure) = found {
        let local_scope = scope.clone();
        let args: Vec<Literal> = call
            .args
            .into_iter()
            .map(|expr| eval(expr, &local_scope))
            .collect();
        for (symbol, val) in closure.params.into_iter().zip(args) {
            // Inject all arguments into local scope
            local_scope.set(&symbol, val);
        }
        eval_program(closure.body, &local_scope)
    } else {
        panic!("Cannot call {}: not callable", call.symbol);
    }
}

pub fn eval<T: Scope + Clone>(expr: Expr, scope: &T) -> Literal {
    match expr {
        Expr::Literal(val) => val,
        Expr::BinaryOp(op) => eval_binary_op(*op, scope),
        Expr::Asignment(asign) => {
            let evaluated = eval(*asign.value, scope);
            scope.set(&asign.symbol, evaluated.clone());
            evaluated
        }
        Expr::Call(call) => eval_call(call, scope),
        Expr::Symbol(symbol) => scope.get(&symbol),
    }
}

#[cfg(test)]
mod tests {
    use ast::{Asignment, Closure, If, While};
    use scope::HashScope;

    use super::*;

    #[test]
    fn eval_primitive() {
        let scope = HashScope::default();
        let result = eval(Expr::Literal(Literal::Int(1)), &scope);
        let expected = Literal::Int(1);
        assert_eq!(result, expected);

        let result = eval(Expr::Literal(Literal::Bool(true)), &scope);
        let expected = Literal::Bool(true);
        assert_eq!(result, expected);

        let result = eval(Expr::Literal(Literal::String(String::from("test"))), &scope);
        let expected = Literal::String(String::from("test"));
        assert_eq!(result, expected);

        let result = eval(Expr::Literal(Literal::Float(1.5)), &scope);
        let expected = Literal::Float(1.5);
        assert_eq!(result, expected);

        eval(
            Expr::Asignment(Asignment {
                symbol: String::from("name"),
                value: Box::new(Expr::Literal(Literal::Int(4))),
            }),
            &scope,
        );
        let symbol = Expr::Symbol(String::from("name"));
        let found_value = eval(symbol, &scope);
        assert_eq!(found_value, Literal::Int(4))
    }
    #[test]
    fn eval_add_operation() {
        let scope = HashScope::default();
        let op = Expr::BinaryOp(Box::new(BinaryOp {
            left: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Literal(Literal::Int(2)),
                Expr::Literal(Literal::Int(8)),
                BinaryOpType::Add,
            ))),
            right: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Literal(Literal::Float(4.5)),
                Expr::Literal(Literal::Int(5)),
                BinaryOpType::Add,
            ))),
            op_type: BinaryOpType::Add,
        }));
        let result = eval(op, &scope);
        let expected = Literal::Float(19.5);
        assert_eq!(result, expected);
    }
    #[test]
    #[should_panic]
    fn try_operate_string() {
        let scope = HashScope::default();

        let op = Expr::BinaryOp(Box::new(BinaryOp::new(
            Expr::Literal(Literal::String(String::from("Gab"))),
            Expr::Literal(Literal::String(String::from("riel"))),
            BinaryOpType::Add,
        )));
        eval(op, &scope);
    }
    #[test]
    fn eval_sub_operation() {
        let scope = HashScope::default();
        let op = Expr::BinaryOp(Box::new(BinaryOp {
            left: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Literal(Literal::Int(8)),
                Expr::Literal(Literal::Int(6)),
                BinaryOpType::Sub,
            ))),
            right: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Literal(Literal::Float(4.5)),
                Expr::Literal(Literal::Float(3.5)),
                BinaryOpType::Sub,
            ))),
            op_type: BinaryOpType::Add,
        }));
        let result = eval(op, &scope);
        let expected = Literal::Float(3.0);
        assert_eq!(result, expected);
    }
    #[test]
    fn eval_multiplication() {
        let scope = HashScope::default();

        let op = Expr::BinaryOp(Box::new(BinaryOp {
            left: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Literal(Literal::Int(2)),
                Expr::Literal(Literal::Int(8)),
                BinaryOpType::Mul,
            ))),
            right: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Literal(Literal::Float(4.5)),
                Expr::Literal(Literal::Int(5)),
                BinaryOpType::Mul,
            ))),
            op_type: BinaryOpType::Add,
        }));
        let result = eval(op, &scope);
        let expected = Literal::Float(38.5);
        assert_eq!(result, expected);
    }
    #[test]
    fn eval_division() {
        let scope = HashScope::default();

        scope.set("age", Literal::Int(10));

        let op = Expr::BinaryOp(Box::new(BinaryOp {
            left: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Symbol(String::from("age")),
                Expr::Literal(Literal::Int(2)),
                BinaryOpType::Div,
            ))),
            right: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Literal(Literal::Int(5)),
                Expr::Literal(Literal::Float(0.5)),
                BinaryOpType::Div,
            ))),
            op_type: BinaryOpType::Add,
        }));
        let result = eval(op, &scope);
        let expected = Literal::Float(15.0);
        assert_eq!(result, expected);
    }
    #[test]
    fn eval_gt() {
        let scope = HashScope::default();

        let op = Expr::BinaryOp(Box::new(BinaryOp::new(
            Expr::Literal(Literal::Int(8)),
            Expr::Literal(Literal::Int(4)),
            BinaryOpType::Gt,
        )));
        let result = eval(op, &scope);
        let expected = Literal::Bool(true);
        assert_eq!(result, expected);
    }
    #[test]
    fn truthy_or_falsy() {
        let scope = HashScope::default();

        assert_eq!(is_truthy(Expr::Literal(Literal::Null), &scope), false);
        assert_eq!(
            is_truthy(Expr::Literal(Literal::String(String::from(""))), &scope),
            false
        );
        assert_eq!(
            is_truthy(Expr::Literal(Literal::String(String::from("Test"))), &scope),
            true
        );
        assert_eq!(is_truthy(Expr::Literal(Literal::Bool(true)), &scope), true);
        assert_eq!(
            is_truthy(Expr::Literal(Literal::Bool(false)), &scope),
            false
        );
        assert_eq!(is_truthy(Expr::Literal(Literal::Int(0)), &scope), false);
        assert_eq!(is_truthy(Expr::Literal(Literal::Int(1)), &scope), true);
        assert_eq!(is_truthy(Expr::Literal(Literal::Float(1.1)), &scope), true);
        assert_eq!(is_truthy(Expr::Literal(Literal::Float(0.0)), &scope), false);
        assert_eq!(
            is_truthy(
                Expr::BinaryOp(Box::new(BinaryOp::new(
                    Expr::Literal(Literal::Int(4)),
                    Expr::Literal(Literal::Int(7)),
                    BinaryOpType::Add
                ))),
                &scope
            ),
            true
        );
        assert_eq!(
            is_truthy(
                Expr::BinaryOp(Box::new(BinaryOp::new(
                    Expr::Literal(Literal::Int(4)),
                    Expr::Literal(Literal::Int(4)),
                    BinaryOpType::Sub
                ))),
                &scope
            ),
            false
        );
    }
    #[test]
    fn logical_operations() {
        let scope = HashScope::default();
        let op = Expr::BinaryOp(Box::new(BinaryOp::new(
            Expr::Literal(Literal::Bool(true)),
            Expr::Literal(Literal::Bool(false)),
            BinaryOpType::Or,
        )));
        assert_eq!(eval(op, &scope), Literal::Bool(true));

        let op = Expr::BinaryOp(Box::new(BinaryOp::new(
            Expr::Literal(Literal::Bool(true)),
            Expr::Literal(Literal::Bool(false)),
            BinaryOpType::And,
        )));
        assert_eq!(eval(op, &scope), Literal::Bool(false));

        let op = Expr::BinaryOp(Box::new(BinaryOp::new(
            Expr::Literal(Literal::Bool(true)),
            Expr::Literal(Literal::Bool(true)),
            BinaryOpType::And,
        )));
        assert_eq!(eval(op, &scope), Literal::Bool(true));

        let op = Expr::BinaryOp(Box::new(BinaryOp::new(
            Expr::Literal(Literal::Bool(false)),
            Expr::Literal(Literal::Bool(false)),
            BinaryOpType::Or,
        )));
        assert_eq!(eval(op, &scope), Literal::Bool(false));
    }
    #[test]
    fn test_eval_call() {
        let scope = HashScope::default();
        scope.set(
            "greet",
            Literal::Closure(ast::Closure {
                params: vec![String::from("name")],
                body: vec![Instruction::Stmt(Stmt::Return(Expr::Symbol(String::from(
                    "name",
                ))))],
            }),
        );
        let call = Expr::Call(Call {
            symbol: String::from("greet"),
            args: vec![Expr::Literal(Literal::String(String::from("John")))],
        });
        let result = eval(call, &scope);
        assert_eq!(result, Literal::String(String::from("John")));
    }
    #[test]
    fn test_if_else() {
        let scope = HashScope::default();
        let is_adult_fn = Closure {
            params: vec![String::from("age")],
            body: vec![Instruction::Stmt(Stmt::If(If {
                cond: Expr::BinaryOp(Box::new(BinaryOp::new(
                    Expr::Symbol(String::from("age")),
                    Expr::Literal(Literal::Int(18)),
                    BinaryOpType::Ge,
                ))),
                body: vec![Instruction::Stmt(Stmt::Return(Expr::Literal(
                    Literal::Bool(true),
                )))],
                else_block: Some(vec![Instruction::Stmt(Stmt::Return(Expr::Literal(
                    Literal::Bool(false),
                )))]),
            }))],
        };
        // Rust equivalent to this function:
        // fn is_adult(age: i64) -> bool {
        //  if age >= 18 {
        //      true
        //  } else {
        //      false
        //  }
        // }
        scope.set("is_adult", Literal::Closure(is_adult_fn));
        let call = Expr::Call(Call {
            symbol: String::from("is_adult"),
            args: vec![Expr::Literal(Literal::Int(18))],
        });
        let result = eval(call, &scope);
        assert_eq!(result, Literal::Bool(true));

        let call = Expr::Call(Call {
            symbol: String::from("is_adult"),
            args: vec![Expr::Literal(Literal::Int(17))],
        });
        let result = eval(call, &scope);
        assert_eq!(result, Literal::Bool(false));
    }
    #[test]
    fn test_while_loop() {
        let scope = HashScope::default();
        scope.set("count", Literal::Int(0));
        let program: Program = vec![Instruction::Stmt(Stmt::While(While {
            cond: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Symbol(String::from("count")),
                Expr::Literal(Literal::Int(10)),
                BinaryOpType::Lt,
            ))),
            body: vec![Instruction::Expr(Expr::Asignment(Asignment {
                symbol: String::from("count"),
                value: Box::new(Expr::BinaryOp(Box::new(BinaryOp::new(
                    Expr::Symbol(String::from("count")),
                    Expr::Literal(Literal::Int(1)),
                    BinaryOpType::Add,
                )))),
            }))],
        }))];
        // Rust equivalent
        // while count < 10 {
        //  count = count + 1;
        // }
        eval_program(program, &scope);
        let final_count = scope.get("count");
        assert_eq!(final_count, Literal::Int(10));
    }
}

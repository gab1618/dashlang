mod scope;

use ast::{BinaryOp, BinaryOpType, Call, Expr, Instruction, Program, Stmt, Value};
use scope::{HashScope, Scope};

macro_rules! define_aritmetic_operation {
    ($operator:tt, $op:expr, $scope:expr) => {
        match ($op.left, $op.right) {
            (Expr::Value(left), Expr::Value(right)) => match (left, right) {
                (Value::Int(left), Value::Int(right)) => Value::Int(left $operator right),
                (Value::Float(left), Value::Int(right)) => Value::Float(left $operator (right as f64)),
                (Value::Int(left), Value::Float(right)) => Value::Float((left as f64) $operator right),
                (Value::Float(left), Value::Float(right)) => Value::Float(left $operator right),
                (_, _) => panic!("Unsuported operation"),
            },
            (left, right) => eval_binary_op(
                BinaryOp::new(
                    Expr::Value(eval(left, $scope)),
                    Expr::Value(eval(right, $scope)),
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
            (Expr::Value(left), Expr::Value(right)) => match (left, right) {
                (Value::Int(left), Value::Int(right)) => Value::Bool(left $operator right),
                (Value::Float(left), Value::Int(right)) => Value::Bool(left $operator (right as f64)),
                (Value::Int(left), Value::Float(right)) => Value::Bool((left as f64) $operator right),
                (Value::Float(left), Value::Float(right)) => Value::Bool(left $operator right),
                (_, _) => panic!("Unsuported operation"),
            },
            (left, right) => eval_binary_op(
                BinaryOp::new(
                    Expr::Value(eval(left, $scope)),
                    Expr::Value(eval(right, $scope)),
                    $op.op_type,
                ),
                $scope,
            ),
        }
    };
}

fn is_truthy(expr: Expr, scope: &mut HashScope) -> bool {
    match expr {
        Expr::Value(value) => match value {
            Value::Closure(_) => true,
            Value::Int(num) => num != 0,
            Value::Float(num) => num != 0.0,
            Value::String(string) => !string.is_empty(),
            Value::Bool(bool) => bool,
            Value::Null => false,
            Value::Void => false,
        },
        expr => is_truthy(Expr::Value(eval(expr, scope)), scope),
    }
}

pub fn eval_binary_op(op: BinaryOp, scope: &mut HashScope) -> Value {
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
        BinaryOpType::And => Value::Bool(is_truthy(op.left, scope) && is_truthy(op.right, scope)),
        BinaryOpType::Or => Value::Bool(is_truthy(op.left, scope) || is_truthy(op.right, scope)),
    }
}

fn eval_program(program: Program, scope: &mut HashScope) -> Value {
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
                            Value::Void => (),
                            val => return val,
                        }
                    } else {
                        if let Some(else_block) = if_stmt.else_block {
                            let block_result = eval_program(else_block, scope);
                            match block_result {
                                Value::Null => (),
                                val => return val,
                            }
                        }
                    }
                }
                Stmt::While(while_stmt) => {
                    while is_truthy(while_stmt.clone().cond, scope) {
                        let block_result = eval_program(while_stmt.clone().body, scope);
                        match block_result {
                            Value::Void => (),
                            val => return val,
                        }
                    }
                }
                Stmt::Print(expr) => {
                    let val = eval(expr, scope);
                    match val {
                        Value::Closure(_) => println!("[closure]"),
                        Value::Int(num) => println!("{num}"),
                        Value::Float(num) => println!("{num}"),
                        Value::String(string) => println!("{string}"),
                        Value::Bool(bool) => {
                            if bool {
                                println!("True")
                            } else {
                                println!("False")
                            }
                        }
                        Value::Null => println!("null"),
                        Value::Void => (),
                    }
                }
            },
            Instruction::Expr(expr) => {
                eval(expr, scope);
            }
        }
    }
    Value::Void
}

fn eval_call(call: Call, scope: &HashScope) -> Value {
    let found = scope.get(call.symbol.clone());
    if let Value::Closure(closure) = found {
        let mut local_scope = scope.clone();
        let args: Vec<Value> = call
            .args
            .into_iter()
            .map(|expr| eval(expr, &mut local_scope))
            .collect();
        for (symbol, val) in closure.params.into_iter().zip(args) {
            // Inject all arguments into local scope
            local_scope.set(symbol.to_string(), val);
        }
        eval_program(closure.body, &mut local_scope)
    } else {
        panic!("Cannot call {}: not callable", call.symbol);
    }
}

pub fn eval(expr: Expr, scope: &mut HashScope) -> Value {
    match expr {
        Expr::Value(val) => val,
        Expr::BinaryOp(op) => eval_binary_op(*op, scope),
        Expr::Asignment(asign) => {
            let evaluated = eval(*asign.value, scope);
            scope.set(asign.symbol, evaluated.clone());
            evaluated
        }
        Expr::Call(call) => eval_call(call, scope),
        Expr::Symbol(symbol) => scope.get(symbol),
    }
}

#[cfg(test)]
mod tests {
    use ast::{Asignment, Closure, If, While};

    use super::*;

    #[test]
    fn eval_primtitive() {
        let mut scope = HashScope::new();
        let result = eval(Expr::Value(Value::Int(1)), &mut scope);
        let expected = Value::Int(1);
        assert_eq!(result, expected);

        let result = eval(Expr::Value(Value::Bool(true)), &mut scope);
        let expected = Value::Bool(true);
        assert_eq!(result, expected);

        let result = eval(Expr::Value(Value::String(String::from("test"))), &mut scope);
        let expected = Value::String(String::from("test"));
        assert_eq!(result, expected);

        let result = eval(Expr::Value(Value::Float(1.5)), &mut scope);
        let expected = Value::Float(1.5);
        assert_eq!(result, expected);

        eval(
            Expr::Asignment(Asignment {
                symbol: String::from("name"),
                value: Box::new(Expr::Value(Value::Int(4))),
            }),
            &mut scope,
        );
        let symbol = Expr::Symbol(String::from("name"));
        let found_value = eval(symbol, &mut scope);
        assert_eq!(found_value, Value::Int(4))
    }
    #[test]
    fn eval_add_operation() {
        let mut scope = HashScope::new();
        let op = Expr::BinaryOp(Box::new(BinaryOp {
            left: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Value(Value::Int(2)),
                Expr::Value(Value::Int(8)),
                BinaryOpType::Add,
            ))),
            right: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Value(Value::Float(4.5)),
                Expr::Value(Value::Int(5)),
                BinaryOpType::Add,
            ))),
            op_type: BinaryOpType::Add,
        }));
        let result = eval(op, &mut scope);
        let expected = Value::Float(19.5);
        assert_eq!(result, expected);
    }
    #[test]
    #[should_panic]
    fn try_operate_string() {
        let mut scope = HashScope::new();

        let op = Expr::BinaryOp(Box::new(BinaryOp::new(
            Expr::Value(Value::String(String::from("Gab"))),
            Expr::Value(Value::String(String::from("riel"))),
            BinaryOpType::Add,
        )));
        eval(op, &mut scope);
    }
    #[test]
    fn eval_sub_operation() {
        let mut scope = HashScope::new();
        let op = Expr::BinaryOp(Box::new(BinaryOp {
            left: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Value(Value::Int(8)),
                Expr::Value(Value::Int(6)),
                BinaryOpType::Sub,
            ))),
            right: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Value(Value::Float(4.5)),
                Expr::Value(Value::Float(3.5)),
                BinaryOpType::Sub,
            ))),
            op_type: BinaryOpType::Add,
        }));
        let result = eval(op, &mut scope);
        let expected = Value::Float(3.0);
        assert_eq!(result, expected);
    }
    #[test]
    fn eval_multiplication() {
        let mut scope = HashScope::new();

        let op = Expr::BinaryOp(Box::new(BinaryOp {
            left: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Value(Value::Int(2)),
                Expr::Value(Value::Int(8)),
                BinaryOpType::Mul,
            ))),
            right: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Value(Value::Float(4.5)),
                Expr::Value(Value::Int(5)),
                BinaryOpType::Mul,
            ))),
            op_type: BinaryOpType::Add,
        }));
        let result = eval(op, &mut scope);
        let expected = Value::Float(38.5);
        assert_eq!(result, expected);
    }
    #[test]
    fn eval_division() {
        let mut scope = HashScope::new();

        scope.set(String::from("age"), Value::Int(10));

        let op = Expr::BinaryOp(Box::new(BinaryOp {
            left: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Symbol(String::from("age")),
                Expr::Value(Value::Int(2)),
                BinaryOpType::Div,
            ))),
            right: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Value(Value::Int(5)),
                Expr::Value(Value::Float(0.5)),
                BinaryOpType::Div,
            ))),
            op_type: BinaryOpType::Add,
        }));
        let result = eval(op, &mut scope);
        let expected = Value::Float(15.0);
        assert_eq!(result, expected);
    }
    #[test]
    fn eval_gt() {
        let mut scope = HashScope::new();

        let op = Expr::BinaryOp(Box::new(BinaryOp::new(
            Expr::Value(Value::Int(8)),
            Expr::Value(Value::Int(4)),
            BinaryOpType::Gt,
        )));
        let result = eval(op, &mut scope);
        let expected = Value::Bool(true);
        assert_eq!(result, expected);
    }
    #[test]
    fn truthy_or_falsy() {
        let mut scope = HashScope::new();

        assert_eq!(is_truthy(Expr::Value(Value::Null), &mut scope), false);
        assert_eq!(
            is_truthy(Expr::Value(Value::String(String::from(""))), &mut scope),
            false
        );
        assert_eq!(
            is_truthy(Expr::Value(Value::String(String::from("Test"))), &mut scope),
            true
        );
        assert_eq!(is_truthy(Expr::Value(Value::Bool(true)), &mut scope), true);
        assert_eq!(
            is_truthy(Expr::Value(Value::Bool(false)), &mut scope),
            false
        );
        assert_eq!(is_truthy(Expr::Value(Value::Int(0)), &mut scope), false);
        assert_eq!(is_truthy(Expr::Value(Value::Int(1)), &mut scope), true);
        assert_eq!(is_truthy(Expr::Value(Value::Float(1.1)), &mut scope), true);
        assert_eq!(is_truthy(Expr::Value(Value::Float(0.0)), &mut scope), false);
        assert_eq!(
            is_truthy(
                Expr::BinaryOp(Box::new(BinaryOp::new(
                    Expr::Value(Value::Int(4)),
                    Expr::Value(Value::Int(7)),
                    BinaryOpType::Add
                ))),
                &mut scope
            ),
            true
        );
        assert_eq!(
            is_truthy(
                Expr::BinaryOp(Box::new(BinaryOp::new(
                    Expr::Value(Value::Int(4)),
                    Expr::Value(Value::Int(4)),
                    BinaryOpType::Sub
                ))),
                &mut scope
            ),
            false
        );
    }
    #[test]
    fn logical_operations() {
        let mut scope = HashScope::new();
        let op = Expr::BinaryOp(Box::new(BinaryOp::new(
            Expr::Value(Value::Bool(true)),
            Expr::Value(Value::Bool(false)),
            BinaryOpType::Or,
        )));
        assert_eq!(eval(op, &mut scope), Value::Bool(true));

        let op = Expr::BinaryOp(Box::new(BinaryOp::new(
            Expr::Value(Value::Bool(true)),
            Expr::Value(Value::Bool(false)),
            BinaryOpType::And,
        )));
        assert_eq!(eval(op, &mut scope), Value::Bool(false));

        let op = Expr::BinaryOp(Box::new(BinaryOp::new(
            Expr::Value(Value::Bool(true)),
            Expr::Value(Value::Bool(true)),
            BinaryOpType::And,
        )));
        assert_eq!(eval(op, &mut scope), Value::Bool(true));

        let op = Expr::BinaryOp(Box::new(BinaryOp::new(
            Expr::Value(Value::Bool(false)),
            Expr::Value(Value::Bool(false)),
            BinaryOpType::Or,
        )));
        assert_eq!(eval(op, &mut scope), Value::Bool(false));
    }
    #[test]
    fn test_eval_call() {
        let mut scope = HashScope::new();
        scope.set(
            String::from("greet"),
            Value::Closure(ast::Closure {
                params: vec![String::from("name")],
                body: vec![Instruction::Stmt(Stmt::Return(Expr::Symbol(String::from(
                    "name",
                ))))],
            }),
        );
        let call = Expr::Call(Call {
            symbol: String::from("greet"),
            args: vec![Expr::Value(Value::String(String::from("John")))],
        });
        let result = eval(call, &mut scope);
        assert_eq!(result, Value::String(String::from("John")));
    }
    #[test]
    fn test_if_else() {
        let mut scope = HashScope::new();
        let is_adult_fn = Closure {
            params: vec![String::from("age")],
            body: vec![Instruction::Stmt(Stmt::If(If {
                cond: Expr::BinaryOp(Box::new(BinaryOp::new(
                    Expr::Symbol(String::from("age")),
                    Expr::Value(Value::Int(18)),
                    BinaryOpType::Ge,
                ))),
                body: vec![Instruction::Stmt(Stmt::Return(Expr::Value(Value::Bool(
                    true,
                ))))],
                else_block: Some(vec![Instruction::Stmt(Stmt::Return(Expr::Value(
                    Value::Bool(false),
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
        scope.set(String::from("is_adult"), Value::Closure(is_adult_fn));
        let call = Expr::Call(Call {
            symbol: String::from("is_adult"),
            args: vec![Expr::Value(Value::Int(18))],
        });
        let result = eval(call, &mut scope);
        assert_eq!(result, Value::Bool(true));

        let call = Expr::Call(Call {
            symbol: String::from("is_adult"),
            args: vec![Expr::Value(Value::Int(17))],
        });
        let result = eval(call, &mut scope);
        assert_eq!(result, Value::Bool(false));
    }
    #[test]
    fn test_while_loop() {
        let mut scope = HashScope::new();
        scope.set(String::from("count"), Value::Int(0));
        let program: Program = vec![Instruction::Stmt(Stmt::While(While {
            cond: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Symbol(String::from("count")),
                Expr::Value(Value::Int(10)),
                BinaryOpType::Lt,
            ))),
            body: vec![Instruction::Expr(Expr::Asignment(Asignment {
                symbol: String::from("count"),
                value: Box::new(Expr::BinaryOp(Box::new(BinaryOp::new(
                    Expr::Symbol(String::from("count")),
                    Expr::Value(Value::Int(1)),
                    BinaryOpType::Add,
                )))),
            }))],
        }))];
        // Rust equivalent
        // while count < 10 {
        //  count = count + 1;
        // }
        eval_program(program, &mut scope);
        let final_count = scope.get(String::from("count"));
        assert_eq!(final_count, Value::Int(10));
    }
}

use std::collections::HashMap;

use ast::{BinaryOp, BinaryOpType, Expr};

pub trait Scope {
    fn get(&self, symbol: String) -> Expr;
    fn set(&mut self, symbol: String, val: Expr);
}
#[derive(Clone)]
struct HashScope {
    memory: HashMap<String, Expr>,
}
impl Scope for HashScope {
    fn get(&self, symbol: String) -> Expr {
        match self.memory.get(&symbol) {
            Some(value) => value.clone(),
            None => Expr::Null,
        }
    }

    fn set(&mut self, symbol: String, val: Expr) {
        self.memory.insert(symbol, val);
    }
}

macro_rules! define_aritmetic_operation {
    ($operator:tt, $op:expr, $scope:expr) => {
        match ($op.left, $op.right) {
            (Expr::Int(left), Expr::Int(right)) => Expr::Int(left $operator right),
            (Expr::Float(left), Expr::Int(right)) => Expr::Float(left $operator (right as f64)),
            (Expr::Int(left), Expr::Float(right)) => Expr::Float((left as f64) $operator right),
            (Expr::Float(left), Expr::Float(right)) => Expr::Float(left $operator right),
            (Expr::String(_), _) => panic!("Unsuported operation"),
            (_, Expr::String(_)) => panic!("Unsuported operation"),
            (left, right) => {
                let new_op = BinaryOp::new(eval(left, $scope), eval(right, $scope), $op.op_type);
                eval_binary_op(new_op, $scope)
            }
        }
    };
}
macro_rules! define_boolean_operation {
    ($operator:tt, $op:expr, $scope:expr) => {
        match ($op.left, $op.right) {
            (Expr::Int(left), Expr::Int(right)) => Expr::Bool(left $operator right),
            (Expr::Float(left), Expr::Float(right)) => Expr::Bool(left $operator right),
            (Expr::Float(left), Expr::Int(right)) => Expr::Bool(left $operator (right as f64)),
            (Expr::Int(left), Expr::Float(right)) => Expr::Bool((left as f64) $operator right),
            (Expr::String(_), _) => panic!("Unsuported operation"),
            (_, Expr::String(_)) => panic!("Unsuported operation"),
            (left, right) => {
                let new_op = BinaryOp::new(eval(left, $scope), eval(right, $scope), $op.op_type);
                eval_binary_op(new_op, $scope)
            }
        }
    };
}

fn is_truthy(expr: Expr, scope: &mut dyn Scope) -> bool {
    match expr {
        Expr::Closure(_) => true,
        Expr::Int(num) => num != 0,
        Expr::Float(num) => num != 0.0,
        Expr::String(string) => !string.is_empty(),
        Expr::Bool(val) => val,
        Expr::Null => false,
        expr => is_truthy(eval(expr, scope), scope),
    }
}

pub fn eval_binary_op(op: BinaryOp, scope: &mut dyn Scope) -> Expr {
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
        BinaryOpType::Or => Expr::Bool(is_truthy(op.left, scope) || is_truthy(op.right, scope)),
        BinaryOpType::And => Expr::Bool(is_truthy(op.left, scope) && is_truthy(op.right, scope)),
    }
}

pub fn eval(expr: Expr, scope: &mut dyn Scope) -> Expr {
    match expr {
        Expr::Int(val) => Expr::Int(val),
        Expr::Float(val) => Expr::Float(val),
        Expr::Null => Expr::Null,
        Expr::String(val) => Expr::String(val),
        Expr::Bool(val) => Expr::Bool(val),
        Expr::BinaryOp(op) => eval_binary_op(*op, scope),
        Expr::AsignmentExpr(asign) => {
            let evaluated = eval(*asign.value, scope);
            scope.set(asign.symbol, evaluated.clone());
            evaluated
        }
        Expr::Closure(_) => todo!(),
        Expr::Call(_) => todo!(),
        Expr::Symbol(symbol) => scope.get(symbol),
    }
}

#[cfg(test)]
mod tests {
    use ast::AsignmentExpr;

    use super::*;

    macro_rules! hash_scope {
        () => {
            HashScope {
                memory: HashMap::new(),
            }
        };
    }
    #[test]
    fn eval_primtitive() {
        let mut scope = hash_scope!();
        let result = eval(Expr::Int(1), &mut scope);
        let expected = Expr::Int(1);
        assert_eq!(result, expected);

        let result = eval(Expr::Bool(true), &mut scope);
        let expected = Expr::Bool(true);
        assert_eq!(result, expected);

        let result = eval(Expr::String(String::from("test")), &mut scope);
        let expected = Expr::String(String::from("test"));
        assert_eq!(result, expected);

        let result = eval(Expr::Float(1.5), &mut scope);
        let expected = Expr::Float(1.5);
        assert_eq!(result, expected);

        eval(
            Expr::AsignmentExpr(AsignmentExpr {
                symbol: String::from("name"),
                value: Box::new(Expr::Int(4)),
            }),
            &mut scope,
        );
        let symbol = Expr::Symbol(String::from("name"));
        let found_value = eval(symbol, &mut scope);
        assert_eq!(found_value, Expr::Int(4))
    }
    #[test]
    fn eval_add_operation() {
        let mut scope = hash_scope!();
        let op = Expr::BinaryOp(Box::new(BinaryOp {
            left: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Int(2),
                Expr::Int(8),
                BinaryOpType::Add,
            ))),
            right: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Float(4.5),
                Expr::Int(5),
                BinaryOpType::Add,
            ))),
            op_type: BinaryOpType::Add,
        }));
        let result = eval(op, &mut scope);
        let expected = Expr::Float(19.5);
        assert_eq!(result, expected);
    }
    #[test]
    #[should_panic]
    fn try_operate_string() {
        let mut scope = hash_scope!();

        let op = Expr::BinaryOp(Box::new(BinaryOp::new(
            Expr::String(String::from("Gab")),
            Expr::String(String::from("riel")),
            BinaryOpType::Add,
        )));
        eval(op, &mut scope);
    }
    #[test]
    fn eval_sub_operation() {
        let mut scope = hash_scope!();
        let op = Expr::BinaryOp(Box::new(BinaryOp {
            left: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Int(8),
                Expr::Int(6),
                BinaryOpType::Sub,
            ))),
            right: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Float(4.5),
                Expr::Float(3.5),
                BinaryOpType::Sub,
            ))),
            op_type: BinaryOpType::Add,
        }));
        let result = eval(op, &mut scope);
        let expected = Expr::Float(3.0);
        assert_eq!(result, expected);
    }
    #[test]
    fn eval_multiplication() {
        let mut scope = hash_scope!();

        let op = Expr::BinaryOp(Box::new(BinaryOp {
            left: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Int(2),
                Expr::Int(8),
                BinaryOpType::Mul,
            ))),
            right: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Float(4.5),
                Expr::Int(5),
                BinaryOpType::Mul,
            ))),
            op_type: BinaryOpType::Add,
        }));
        let result = eval(op, &mut scope);
        let expected = Expr::Float(38.5);
        assert_eq!(result, expected);
    }
    #[test]
    fn eval_division() {
        let mut scope = hash_scope!();

        scope.set(String::from("age"), Expr::Int(10));

        let op = Expr::BinaryOp(Box::new(BinaryOp {
            left: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Symbol(String::from("age")),
                Expr::Int(2),
                BinaryOpType::Div,
            ))),
            right: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Int(5),
                Expr::Float(0.5),
                BinaryOpType::Div,
            ))),
            op_type: BinaryOpType::Add,
        }));
        let result = eval(op, &mut scope);
        let expected = Expr::Float(15.0);
        assert_eq!(result, expected);
    }
    #[test]
    fn eval_gt() {
        let mut scope = hash_scope!();

        let op = Expr::BinaryOp(Box::new(BinaryOp::new(
            Expr::Int(8),
            Expr::Int(4),
            BinaryOpType::Gt,
        )));
        let result = eval(op, &mut scope);
        let expected = Expr::Bool(true);
        assert_eq!(result, expected);
    }
    #[test]
    fn truthy_or_falsy() {
        let mut scope = hash_scope!();

        assert_eq!(is_truthy(Expr::Null, &mut scope), false);
        assert_eq!(is_truthy(Expr::String(String::from("")), &mut scope), false);
        assert_eq!(
            is_truthy(Expr::String(String::from("Test")), &mut scope),
            true
        );
        assert_eq!(is_truthy(Expr::Bool(true), &mut scope), true);
        assert_eq!(is_truthy(Expr::Bool(false), &mut scope), false);
        assert_eq!(is_truthy(Expr::Int(0), &mut scope), false);
        assert_eq!(is_truthy(Expr::Int(1), &mut scope), true);
        assert_eq!(is_truthy(Expr::Float(1.1), &mut scope), true);
        assert_eq!(is_truthy(Expr::Float(0.0), &mut scope), false);
        assert_eq!(
            is_truthy(
                Expr::BinaryOp(Box::new(BinaryOp::new(
                    Expr::Int(4),
                    Expr::Int(7),
                    BinaryOpType::Add
                ))),
                &mut scope
            ),
            true
        );
        assert_eq!(
            is_truthy(
                Expr::BinaryOp(Box::new(BinaryOp::new(
                    Expr::Int(4),
                    Expr::Int(4),
                    BinaryOpType::Sub
                ))),
                &mut scope
            ),
            false
        );
    }
    #[test]
    fn logical_operations() {
        let mut scope = hash_scope!();
        let op = Expr::BinaryOp(Box::new(BinaryOp::new(
            Expr::Bool(true),
            Expr::Bool(false),
            BinaryOpType::Or,
        )));
        assert_eq!(eval(op, &mut scope), Expr::Bool(true));

        let op = Expr::BinaryOp(Box::new(BinaryOp::new(
            Expr::Bool(true),
            Expr::Bool(false),
            BinaryOpType::And,
        )));
        assert_eq!(eval(op, &mut scope), Expr::Bool(false));

        let op = Expr::BinaryOp(Box::new(BinaryOp::new(
            Expr::Bool(true),
            Expr::Bool(true),
            BinaryOpType::And,
        )));
        assert_eq!(eval(op, &mut scope), Expr::Bool(true));

        let op = Expr::BinaryOp(Box::new(BinaryOp::new(
            Expr::Bool(false),
            Expr::Bool(false),
            BinaryOpType::Or,
        )));
        assert_eq!(eval(op, &mut scope), Expr::Bool(false));
    }
}

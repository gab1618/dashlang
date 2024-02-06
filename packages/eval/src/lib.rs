use std::collections::HashMap;

use tokens::{BinaryOp, BinaryOpType, Expr};

pub trait Scope {
    fn get(&self, symbol: String) -> Expr;
    fn set(&mut self, symbol: String, val: Expr);
}
#[derive(Clone)]
struct HashScope {
    memory: HashMap<String, Expr>,
}
impl HashScope {
    fn new() -> Self {
        HashScope {
            memory: HashMap::new(),
        }
    }
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
    ($operator:tt, $op:expr) => {
        match ($op.left, $op.right) {
            (Expr::Int(left), Expr::Int(right)) => Expr::Int(left $operator right),
            (Expr::Float(left), Expr::Int(right)) => Expr::Float(left $operator (right as f64)),
            (Expr::Int(left), Expr::Float(right)) => Expr::Float((left as f64) $operator right),
            (Expr::Float(left), Expr::Float(right)) => Expr::Float(left $operator right),
            (Expr::BinaryOp(left), right) => {
                let new_op = BinaryOp::new(eval_binary_op(*left), right, $op.op_type);
                eval_binary_op(new_op)
            }
            (left, Expr::BinaryOp(right)) => {
                let new_op = BinaryOp::new(left, eval_binary_op(*right), $op.op_type);
                eval_binary_op(new_op)
            }
            _ => Expr::Null,
        }
    };
}
macro_rules! define_boolean_operation {
    ($operator:tt, $op:expr) => {
        match ($op.left, $op.right) {
            (Expr::Int(left), Expr::Int(right)) => Expr::Bool(left $operator right),
            (Expr::Float(left), Expr::Float(right)) => Expr::Bool(left $operator right),
            (Expr::Float(left), Expr::Int(right)) => Expr::Bool(left $operator (right as f64)),
            (Expr::Int(left), Expr::Float(right)) => Expr::Bool((left as f64) $operator right),
            (Expr::Bool(left), Expr::Bool(right)) => Expr::Bool(left $operator right),
            (Expr::BinaryOp(left), right) => {
                let sub_op = BinaryOp::new(eval_binary_op(*left), right, $op.op_type);
                eval_binary_op(sub_op)
            }
            (left, Expr::BinaryOp(right)) => {
                let sub_op = BinaryOp::new(left, eval_binary_op(*right), $op.op_type);
                eval_binary_op(sub_op)
            }
            _ => Expr::Null,
        }
    };
}

pub fn eval_binary_op(op: BinaryOp) -> Expr {
    match op.op_type {
        BinaryOpType::Add => define_aritmetic_operation!(+, op),
        BinaryOpType::Sub => define_aritmetic_operation!(-, op),
        BinaryOpType::Mul => define_aritmetic_operation!(*, op),
        BinaryOpType::Div => define_aritmetic_operation!(/, op),
        BinaryOpType::Gt => define_boolean_operation!(>, op),
        BinaryOpType::Eq => define_boolean_operation!(==, op),
        BinaryOpType::Ge => define_boolean_operation!(>=, op),
        BinaryOpType::Lt => define_boolean_operation!(<, op),
        BinaryOpType::Le => define_boolean_operation!(<=, op),
    }
}

pub fn eval(expr: Expr, scope: &mut dyn Scope) -> Expr {
    match expr {
        Expr::Int(val) => Expr::Int(val),
        Expr::Float(val) => Expr::Float(val),
        Expr::Null => Expr::Null,
        Expr::String(val) => Expr::String(val),
        Expr::Bool(val) => Expr::Bool(val),
        Expr::BinaryOp(op) => eval_binary_op(*op),
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
    use tokens::AsignmentExpr;

    use super::*;
    #[test]
    fn eval_primtitive() {
        let mut scope = HashScope::new();
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
        let mut scope = HashScope::new();
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
    fn eval_sub_operation() {
        let mut scope = HashScope::new();
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
        let mut scope = HashScope::new();

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
        let mut scope = HashScope::new();

        let op = Expr::BinaryOp(Box::new(BinaryOp {
            left: Expr::BinaryOp(Box::new(BinaryOp::new(
                Expr::Int(6),
                Expr::Int(3),
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
        let expected = Expr::Float(12.0);
        assert_eq!(result, expected);
    }
    #[test]
    fn eval_gt() {
        let mut scope = HashScope::new();

        let op = Expr::BinaryOp(Box::new(BinaryOp::new(
            Expr::Int(8),
            Expr::Int(4),
            BinaryOpType::Gt,
        )));
        let result = eval(op, &mut scope);
        let expected = Expr::Bool(true);
        assert_eq!(result, expected);
    }
}

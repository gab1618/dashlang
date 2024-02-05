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
            (Expr::Float(left), Expr::Int(right)) => Expr::Float(left $operator right as f64),
            (Expr::Int(left), Expr::Float(right)) => Expr::Float(left as f64 $operator right),
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

pub fn eval_binary_op(op: BinaryOp) -> Expr {
    match op.op_type {
        BinaryOpType::Add => define_aritmetic_operation!(+, op),
        BinaryOpType::Sub => define_aritmetic_operation!(-, op),
        BinaryOpType::Mul => define_aritmetic_operation!(*, op),
        BinaryOpType::Div => define_aritmetic_operation!(/, op),
        BinaryOpType::Gt => todo!(),
        BinaryOpType::Eq => todo!(),
        BinaryOpType::Ge => todo!(),
        BinaryOpType::Lt => todo!(),
        BinaryOpType::Le => todo!(),
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
            left: Expr::BinaryOp(Box::new(BinaryOp {
                left: Expr::Int(2),
                right: Expr::Int(8),
                op_type: BinaryOpType::Add,
            })),
            right: Expr::Int(8),
            op_type: BinaryOpType::Add,
        }));
        let result = eval(op, &mut scope);
        let expected = Expr::Int(18);
        assert_eq!(result, expected);
    }
}

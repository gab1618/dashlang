use std::collections::HashMap;

use tokens::Expr;

pub trait Scope {
    fn get(&self, symbol: String) -> Expr;
    fn set(&mut self, symbol: String, val: Expr);
}
#[derive(Clone)]
struct SimpleScope {
    memory: HashMap<String, Expr>,
}
impl SimpleScope {
    fn new() -> Self {
        SimpleScope {
            memory: HashMap::new(),
        }
    }
}
impl Scope for SimpleScope {
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

pub fn eval(expr: Expr, scope: &mut dyn Scope) -> Expr {
    match expr {
        Expr::Int(val) => Expr::Int(val),
        Expr::Float(val) => Expr::Float(val),
        Expr::Null => Expr::Null,
        Expr::String(val) => Expr::String(val),
        Expr::Bool(val) => Expr::Bool(val),
        Expr::BinaryOp(op) => match op.op_type {
            tokens::BinaryOpType::Add => todo!(),
            tokens::BinaryOpType::Sub => todo!(),
            tokens::BinaryOpType::Mul => todo!(),
            tokens::BinaryOpType::Div => todo!(),
            tokens::BinaryOpType::Gt => todo!(),
            tokens::BinaryOpType::Eq => todo!(),
            tokens::BinaryOpType::Ge => todo!(),
            tokens::BinaryOpType::Lt => todo!(),
            tokens::BinaryOpType::Le => todo!(),
        },
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
        let mut scope = SimpleScope::new();
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
}

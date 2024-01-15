use tokens::{Expr, Primitive};

pub fn eval(expr: Expr) -> Primitive {
    match expr {
        Expr::Primitive(value) => {
            return value;
        }
        Expr::BinaryOp(_) => todo!(),
        Expr::AsignmentExpr(_) => todo!(),
        Expr::Closure(_) => todo!(),
        Expr::Call(_) => todo!(),
        Expr::Symbol(_) => todo!(),
        Expr::Void => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn eval_primtitive() {
        let result = eval(Expr::Primitive(Primitive::Int(1)));
        let expected = Primitive::Int(1);
        assert_eq!(result, expected);

        let result = eval(Expr::Primitive(Primitive::Bool(true)));
        let expected = Primitive::Bool(true);
        assert_eq!(result, expected);

        let result = eval(Expr::Primitive(Primitive::String(String::from("test"))));
        let expected = Primitive::String(String::from("test"));
        assert_eq!(result, expected);

        let result = eval(Expr::Primitive(Primitive::Float(1.5)));
        let expected = Primitive::Float(1.5);
        assert_eq!(result, expected);
    }
}

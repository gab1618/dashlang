use ast::{Expr, Literal};

pub fn stdlib_push(base: Literal, item: Literal) -> Literal {
    match base {
        Literal::String(mut val) => {
            if let Literal::String(str_push) = item {
                val.push_str(&str_push);
                return Literal::String(val);
            }
            panic!("Unsuported operation");
        }
        Literal::Vector(mut vector) => {
            vector.push(Expr::Literal(item));
            return Literal::Vector(vector);
        }
        _ => panic!("Unsuported operation"),
    }
}

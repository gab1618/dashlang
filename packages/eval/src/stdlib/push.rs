use ast::{Expr, Literal};

use crate::errors::{RuntimeError, RuntimeResult};

pub fn stdlib_push(base: Literal, item: Literal) -> RuntimeResult<Literal> {
    match base {
        Literal::String(mut val) => match item {
            Literal::String(str_push) => {
                val.push_str(&str_push);
                Ok(Literal::String(val))
            }
            _ => Err(RuntimeError::new("Unsuported operation")),
        },
        Literal::Vector(mut vector) => {
            vector.push(Expr::Literal(item));
            Ok(Literal::Vector(vector))
        }
        _ => Err(RuntimeError::new("Unsuported operation")),
    }
}

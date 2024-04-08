use ast::{Expr, Literal};

use crate::errors::{RuntimeError, RuntimeResult};

pub fn stdlib_push(base: Literal, item: Literal) -> RuntimeResult<Literal> {
    match base {
        Literal::String(mut val) => {
            if let Literal::String(str_push) = item {
                val.push_str(&str_push);
                return Ok(Literal::String(val));
            }
            panic!("Unsuported operation");
        }
        Literal::Vector(mut vector) => {
            vector.push(Expr::Literal(item));
            return Ok(Literal::Vector(vector));
        }
        _ => Err(RuntimeError::new("Unsuported operation")),
    }
}

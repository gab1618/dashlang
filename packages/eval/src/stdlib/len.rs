use ast::Literal;

use crate::errors::{RuntimeError, RuntimeErrorKind, RuntimeResult};

pub fn stdlib_len(item: Literal) -> RuntimeResult<Literal> {
    match item {
        Literal::String(val) => Ok(Literal::Int(val.len() as i64)),
        Literal::Vector(val) => Ok(Literal::Int(val.len() as i64)),
        _ => Err(RuntimeError::new(
            "Could not get length: unsuported operation",
            RuntimeErrorKind::NonCallableError,
        )),
    }
}

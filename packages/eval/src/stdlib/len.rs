use ast::{Int, Literal, Location};

use crate::errors::{RuntimeError, RuntimeResult};

pub fn stdlib_len(item: Literal) -> RuntimeResult<Literal> {
    match item {
        Literal::String(val) => Ok(Literal::Int(ast::Int {
            value: val.value.len() as i64,
            location: Location::default(),
        })),
        Literal::Vector(val) => Ok(Literal::Int(Int {
            value: val.value.len() as i64,
            location: Default::default(),
        })),
        _ => Err(RuntimeError::new(
            "Could not get length: unsuported operation",
        )),
    }
}

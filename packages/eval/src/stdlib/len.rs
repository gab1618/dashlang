use std::{fmt::Debug, path::Path};

use ast::{Call, Int, Literal, Location};

use crate::errors::RuntimeError;

pub fn stdlib_len<P: AsRef<Path> + Debug>(
    item: Literal,
    source_path: P,
    call: Call,
) -> Result<Literal, RuntimeError<P>> {
    match item {
        Literal::String(val) => Ok(Literal::Int(ast::Int {
            value: val.value.len() as i64,
            location: Location::default(),
        })),
        Literal::Vector(val) => Ok(Literal::Int(Int {
            value: val.value.len() as i64,
            location: Default::default(),
        })),
        _ => Err(
            RuntimeError::new("Could not get length: unsuported operation")
                .location(call.location, source_path),
        ),
    }
}

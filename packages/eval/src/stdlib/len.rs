use ast::{Call, Int, Literal, Location};
use errors::{DashlangError, DashlangResult, ErrorKind, RuntimeErrorKind};

pub fn stdlib_len(item: Literal, call: Call) -> DashlangResult<Literal> {
    match item {
        Literal::String(val) => Ok(Literal::Int(ast::Int {
            value: val.value.len() as i64,
            location: Location::default(),
        })),
        Literal::Vector(val) => Ok(Literal::Int(Int {
            value: val.value.len() as i64,
            location: Default::default(),
        })),
        _ => Err(DashlangError::new(
            "Could not get length: unsuported operation",
            ErrorKind::Runtime(RuntimeErrorKind::Default),
        )
        .location(call.location)),
    }
}

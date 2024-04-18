use std::io;

use ast::{Call, Literal, Location};
use errors::{DashlangError, DashlangResult, ErrorKind, RuntimeErrorKind};

pub fn stdlib_input(call: Call) -> DashlangResult<Literal> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(Literal::String(ast::Str {
            value: input,
            location: Location::default(),
        })),
        Err(_) => Err(DashlangError::new(
            "Could not get input",
            ErrorKind::Runtime(RuntimeErrorKind::Default),
        )
        .location(call.location)),
    }
}

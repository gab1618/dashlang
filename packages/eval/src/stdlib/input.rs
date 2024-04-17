use std::io;

use ast::{Call, Literal, Location};

use crate::errors::{RuntimeError, RuntimeErrorKind};

pub fn stdlib_input(call: Call) -> Result<Literal, RuntimeError> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(Literal::String(ast::Str {
            value: input,
            location: Location::default(),
        })),
        Err(_) => Err(
            RuntimeError::new("Could not get input", RuntimeErrorKind::Default)
                .location(call.location),
        ),
    }
}

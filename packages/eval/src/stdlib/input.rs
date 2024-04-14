use std::{fmt::Debug, io, path::Path};

use ast::{Call, Literal, Location};

use crate::errors::RuntimeError;

pub fn stdlib_input<P: AsRef<Path> + Debug>(
    source_path: P,
    call: Call,
) -> Result<Literal, RuntimeError<P>> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(Literal::String(ast::Str {
            value: input,
            location: Location::default(),
        })),
        Err(_) => {
            Err(RuntimeError::new("Could not get input").location(call.location, source_path))
        }
    }
}

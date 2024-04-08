use std::io;

use ast::Literal;

use crate::errors::{RuntimeError, RuntimeResult};

pub fn stdlib_input() -> RuntimeResult<Literal> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(Literal::String(input)),
        Err(_) => Err(RuntimeError::new("Could not get input")),
    }
}

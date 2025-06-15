use std::io;

use ast::{Call, Literal};
use errors::{DashlangError, DashlangResult, ErrorKind};

use crate::{scope::Scope, Context};

pub fn stdlib_input<T: Scope>(_ctx: &Context<T>, call: Call) -> DashlangResult<Literal> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(Literal::String(ast::Str {
            value: input,
            location: call.location,
        })),
        Err(_) => Err(
            DashlangError::new("Could not get input", ErrorKind::Unknown).location(call.location),
        ),
    }
}

use std::fmt::Debug;
use std::path::Path;

use ast::{Call, Expr, Literal};

use crate::errors::RuntimeError;

pub fn stdlib_push<P: AsRef<Path> + Debug>(
    base: Literal,
    item: Literal,
    source_path: P,
    call: Call,
) -> Result<Literal, RuntimeError<P>> {
    match base {
        Literal::String(mut val) => match item {
            Literal::String(str_push) => {
                val.value.push_str(&str_push.value);
                Ok(Literal::String(val))
            }
            _ => {
                Err(RuntimeError::new("Unsuported operation").location(call.location, source_path))
            }
        },
        Literal::Vector(mut vector) => {
            vector.value.push(Expr::Literal(item));
            Ok(Literal::Vector(vector))
        }
        _ => Err(RuntimeError::new("Unsuported operation").location(call.location, source_path)),
    }
}

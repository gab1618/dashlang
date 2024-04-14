use std::{fmt::Debug, path::Path};

use ast::{Call, Literal};

use crate::{errors::RuntimeError, eval, scope::Scope, Context};

pub fn stdlib_nth<T: Scope + Clone, P: AsRef<Path> + Clone + Debug>(
    value: Literal,
    index: Literal,
    ctx: &Context<T, P>,
    source_path: P,
    call: Call,
) -> Result<Literal, RuntimeError<P>> {
    if let Literal::Int(int_index) = index {
        if let Literal::Vector(vec) = value {
            if (int_index.value as usize) < vec.value.len() {
                return eval(
                    vec.value[int_index.value as usize].clone(),
                    ctx,
                    source_path,
                );
            }
            return Err(RuntimeError::new(
                "Index out of bound",
                call.location,
                source_path,
            ));
        }
        return Err(RuntimeError::new(
            "Expected vector to be indexed",
            call.location,
            source_path,
        ));
    }
    Err(RuntimeError::new(
        "Expected integer to index vector",
        call.location,
        source_path,
    ))
}

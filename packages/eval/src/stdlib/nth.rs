use ast::{Call, Literal};

use crate::{
    errors::{RuntimeError, RuntimeErrorKind},
    eval,
    scope::Scope,
    Context,
};

pub fn stdlib_nth<T: Scope + Clone>(
    value: Literal,
    index: Literal,
    ctx: &Context<T>,
    call: Call,
) -> Result<Literal, RuntimeError> {
    if let Literal::Int(int_index) = index {
        if let Literal::Vector(vec) = value {
            if (int_index.value as usize) < vec.value.len() {
                return eval(vec.value[int_index.value as usize].clone(), ctx);
            }
            return Err(
                RuntimeError::new("Index out of bound", RuntimeErrorKind::Default)
                    .location(call.location),
            );
        }
        return Err(
            RuntimeError::new("Expected vector to be indexed", RuntimeErrorKind::Default)
                .location(call.location),
        );
    }
    Err(RuntimeError::new(
        "Expected integer to index vector",
        RuntimeErrorKind::Default,
    )
    .location(call.location))
}

use ast::{Call, Literal};
use errors::{DashlangError, DashlangResult, ErrorKind, RuntimeErrorKind};

use crate::{eval, scope::Scope, Context};

pub fn stdlib_nth<T: Scope + Clone>(
    value: Literal,
    index: Literal,
    ctx: &Context<T>,
    call: Call,
) -> DashlangResult<Literal> {
    if let Literal::Int(int_index) = index {
        if let Literal::Vector(vec) = value {
            if (int_index.value as usize) < vec.value.len() {
                return eval(vec.value[int_index.value as usize].clone(), ctx);
            }
            return Err(DashlangError::new(
                "Index out of bound",
                ErrorKind::Runtime(RuntimeErrorKind::Default),
            )
            .location(call.location));
        }
        return Err(DashlangError::new(
            "Index out of bound",
            ErrorKind::Runtime(RuntimeErrorKind::Default),
        )
        .location(call.location));
    }
    return Err(DashlangError::new(
        "Index out of bound",
        ErrorKind::Runtime(RuntimeErrorKind::Default),
    )
    .location(call.location));
}

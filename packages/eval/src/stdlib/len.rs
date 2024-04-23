use ast::{Call, Int, Literal};
use errors::{DashlangError, DashlangResult, ErrorKind, RuntimeErrorKind};

use crate::{eval, scope::Scope, Context};

pub fn stdlib_len<T: Scope + Clone>(ctx: &Context<T>, call: Call) -> DashlangResult<Literal> {
    let mut iter_args = call.args.into_iter();
    let item = eval(
        iter_args.next().ok_or(DashlangError::new(
            "Expected 'item' arg",
            ErrorKind::Runtime(RuntimeErrorKind::WrongArgs),
        ))?,
        ctx,
    )?;
    match item {
        Literal::String(val) => Ok(Literal::Int(ast::Int {
            value: val.value.len() as i64,
            location: call.location,
        })),
        Literal::Vector(val) => Ok(Literal::Int(Int {
            value: val.value.len() as i64,
            location: call.location,
        })),
        _ => Err(DashlangError::new(
            "Could not get length: unsuported operation",
            ErrorKind::Runtime(RuntimeErrorKind::Default),
        )
        .location(call.location)),
    }
}

use ast::{Call, Int, Literal, Location};
use errors::{DashlangError, DashlangResult, ErrorKind, RuntimeErrorKind};

use crate::{eval, scope::Scope, Context};

pub fn stdlib_len<T: Scope + Clone>(ctx: &Context<T>, call: Call) -> DashlangResult<Literal> {
    let mut iter_args = call.args.into_iter();
    let item = eval(iter_args.next().unwrap(), ctx)?;
    match item {
        Literal::String(val) => Ok(Literal::Int(ast::Int {
            value: val.value.len() as i64,
            location: Location::default(),
        })),
        Literal::Vector(val) => Ok(Literal::Int(Int {
            value: val.value.len() as i64,
            location: Default::default(),
        })),
        _ => Err(DashlangError::new(
            "Could not get length: unsuported operation",
            ErrorKind::Runtime(RuntimeErrorKind::Default),
        )
        .location(call.location)),
    }
}

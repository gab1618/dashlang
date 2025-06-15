use ast::{Call, Expr, Literal};
use errors::{DashlangError, DashlangResult, ErrorKind};

use crate::{eval, scope::Scope, Context};

pub fn stdlib_map_set<T: Scope + Clone>(ctx: &Context<T>, call: Call) -> DashlangResult<Literal> {
    let mut iter_args = call.args.into_iter();
    let arg_map = iter_args.next().ok_or_else(|| {
        DashlangError::new(
            "Expected 'map' arg, but none was provided",
            ErrorKind::WrongArgs,
        )
        .location(call.location)
    })?;
    let arg_key = iter_args.next().ok_or_else(|| {
        DashlangError::new(
            "Expected 'key' arg, but none was provided",
            ErrorKind::WrongArgs,
        )
        .location(call.location)
    })?;
    let arg_value = iter_args.next().ok_or_else(|| {
        DashlangError::new(
            "Expected 'value' arg, but none was provided",
            ErrorKind::WrongArgs,
        )
        .location(call.location)
    })?;
    let map = eval(arg_map, ctx)?;
    if let Literal::Map(mut lit_map) = map {
        if let Expr::Literal(Literal::String(key)) = arg_key {
            lit_map.value.insert(key.value, arg_value);
            return Ok(Literal::Map(lit_map));
        }
        return Err(
            DashlangError::new("Expected key to be string", ErrorKind::WrongArgs)
                .location(call.location),
        );
    }
    Err(
        DashlangError::new("Expected arg to be a map", ErrorKind::WrongArgs)
            .location(call.location),
    )
}

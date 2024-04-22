use ast::{Call, Expr, Literal};
use errors::{DashlangError, DashlangResult, ErrorKind, RuntimeErrorKind};

use crate::{eval, scope::Scope, Context};

pub fn stdlib_map_set<T: Scope + Clone>(ctx: &Context<T>, call: Call) -> DashlangResult<Literal> {
    let mut iter_args = call.args.into_iter();
    let arg_map = iter_args.next().expect("Could not get call map arg");
    let arg_key = iter_args.next().expect("Could not get call map key");
    let arg_value = iter_args.next().expect("Could not get call map value");
    let map = eval(arg_map, ctx)?;
    if let Literal::Map(mut lit_map) = map {
        if let Expr::Literal(Literal::String(key)) = arg_key {
            lit_map.value.insert(key.value, arg_value);
            return Ok(Literal::Map(lit_map));
        }
        return Err(DashlangError::new(
            "Expected key to be string",
            ErrorKind::Runtime(RuntimeErrorKind::WrongArgs),
        )
        .location(call.location));
    }
    Err(DashlangError::new(
        "Expected arg to be a map",
        ErrorKind::Runtime(RuntimeErrorKind::WrongArgs),
    )
    .location(call.location))
}

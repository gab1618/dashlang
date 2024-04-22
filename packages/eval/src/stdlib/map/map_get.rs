use ast::{Call, Literal, Null};
use errors::{DashlangError, DashlangResult, ErrorKind, RuntimeErrorKind};

use crate::{eval, scope::Scope, Context};

pub fn stdlib_map_get<T: Scope + Clone>(ctx: &Context<T>, call: Call) -> DashlangResult<Literal> {
    let mut iter_args = call.args.into_iter();
    let map_arg = iter_args.next().expect("Expected map arg");
    let map_arg_location = map_arg.get_location();
    let key_arg = iter_args.next().expect("Expected key arg");
    let key_arg_location = key_arg.get_location();

    if let Literal::Map(map) = eval(map_arg, ctx)? {
        if let Literal::String(key) = eval(key_arg, ctx)? {
            match map.value.get(&key.value) {
                Some(found) => return Ok(eval(found.clone(), ctx)?),
                None => {
                    return Ok(Literal::Null(Null {
                        location: call.location,
                    }))
                }
            }
        }
        return Err(DashlangError::new(
            "Expected argument to be a string",
            ErrorKind::Runtime(RuntimeErrorKind::WrongArgs),
        )
        .location(key_arg_location));
    }
    Err(DashlangError::new(
        "Expected argument to be a map",
        ErrorKind::Runtime(RuntimeErrorKind::WrongArgs),
    )
    .location(map_arg_location))
}

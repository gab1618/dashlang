use ast::{Call, Expr, Literal};
use errors::{DashlangError, DashlangResult, ErrorKind, RuntimeErrorKind};

use crate::{eval, scope::Scope, Context};

pub fn stdlib_push<T: Scope + Clone>(ctx: &Context<T>, call: Call) -> DashlangResult<Literal> {
    let mut iter_args = call.args.into_iter();
    let base = eval(iter_args.next().unwrap(), ctx)?;
    let item = eval(iter_args.next().unwrap(), ctx)?;
    match base {
        Literal::String(mut val) => match item {
            Literal::String(str_push) => {
                val.value.push_str(&str_push.value);
                Ok(Literal::String(val))
            }
            _ => Err(DashlangError::new(
                "Unsuported operation",
                ErrorKind::Runtime(RuntimeErrorKind::Default),
            )
            .location(call.location)),
        },
        Literal::Vector(mut vector) => {
            vector.value.push(Expr::Literal(item));
            Ok(Literal::Vector(vector))
        }
        _ => Err(DashlangError::new(
            "Unsuported operation",
            ErrorKind::Runtime(RuntimeErrorKind::Default),
        )
        .location(call.location)),
    }
}

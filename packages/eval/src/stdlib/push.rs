use ast::{Call, Expr, Literal};
use errors::{DashlangError, DashlangResult, ErrorKind};

use crate::{eval, scope::Scope, Context};

pub fn stdlib_push<T: Scope + Clone>(ctx: &Context<T>, call: Call) -> DashlangResult<Literal> {
    let mut iter_args = call.args.into_iter();
    let base = eval(
        iter_args.next().ok_or(
            DashlangError::new("Expected 'base' argument", ErrorKind::WrongArgs)
                .location(call.location),
        )?,
        ctx,
    )?;
    let item = eval(
        iter_args.next().ok_or(DashlangError::new(
            "Expected 'item' argument",
            ErrorKind::WrongArgs,
        ))?,
        ctx,
    )?;
    match base {
        Literal::String(mut val) => match item {
            Literal::String(str_push) => {
                val.value.push_str(&str_push.value);
                Ok(Literal::String(val))
            }
            _ => Err(
                DashlangError::new("Unsuported operation", ErrorKind::Unknown)
                    .location(call.location),
            ),
        },
        Literal::Vector(mut vector) => {
            vector.value.push(Expr::Literal(item));
            Ok(Literal::Vector(vector))
        }
        _ => Err(
            DashlangError::new("Unsuported operation", ErrorKind::Unknown).location(call.location),
        ),
    }
}

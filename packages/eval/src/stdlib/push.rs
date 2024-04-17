use ast::{Call, Expr, Literal};

use crate::errors::{RuntimeError, RuntimeErrorKind};

pub fn stdlib_push(base: Literal, item: Literal, call: Call) -> Result<Literal, RuntimeError> {
    match base {
        Literal::String(mut val) => match item {
            Literal::String(str_push) => {
                val.value.push_str(&str_push.value);
                Ok(Literal::String(val))
            }
            _ => Err(
                RuntimeError::new("Unsuported operation", RuntimeErrorKind::Default)
                    .location(call.location),
            ),
        },
        Literal::Vector(mut vector) => {
            vector.value.push(Expr::Literal(item));
            Ok(Literal::Vector(vector))
        }
        _ => Err(
            RuntimeError::new("Unsuported operation", RuntimeErrorKind::Default)
                .location(call.location),
        ),
    }
}

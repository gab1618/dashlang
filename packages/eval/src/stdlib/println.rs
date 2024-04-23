use ast::{Call, Literal};
use errors::{DashlangError, DashlangResult, ErrorKind};

use crate::{eval, scope::Scope, Context};

fn stdlib_literal_display<T: Scope + Clone>(
    value: &Literal,
    ctx: &Context<T>,
) -> Result<String, DashlangError> {
    match value {
        Literal::Closure(_) => Ok("Closure".to_string()),
        Literal::Int(val) => Ok(format!("{}", val.value)),
        Literal::Float(val) => Ok(format!("{}", val.value)),
        Literal::String(val) => Ok(val.clone().value),
        Literal::Bool(val) => Ok(if val.value {
            "True".to_string()
        } else {
            "False".to_string()
        }),
        Literal::Vector(val) => {
            let display_args: Result<Vec<String>, DashlangError> = val
                .clone()
                .value
                .into_iter()
                .map(|item| stdlib_literal_display(&eval(item.clone(), ctx)?, ctx))
                .collect();
            match display_args {
                Ok(args) => Ok(format!("[{}]", args.join(", "))),
                Err(err) => Err(err),
            }
        }
        Literal::Null(_) => Ok("Null".to_string()),
        Literal::Void(_) => Ok("Void".to_string()),
        Literal::Tuple(tup) => {
            let display_values: DashlangResult<Vec<String>> = tup
                .clone()
                .value
                .into_iter()
                .map(|item| stdlib_literal_display(&eval(item.clone(), ctx)?, ctx))
                .collect();
            match display_values {
                Ok(args) => Ok(format!("({})", args.join(", "))),
                Err(err) => Err(err),
            }
        }
        Literal::Map(map) => {
            let mut formated_attributes: Vec<String> = vec![];
            for (symbol, value) in map.value.iter() {
                formated_attributes.push(format!(
                    "{symbol}: {}",
                    stdlib_literal_display(&eval(value.clone(), ctx)?, ctx)?
                ));
            }
            Ok(format!("{{ {} }}", formated_attributes.join(", ")))
        }
    }
}

pub fn stdlib_println<T: Scope + Clone>(
    call: Call,
    ctx: &Context<T>,
) -> Result<Literal, DashlangError> {
    let mut iter_args = call.args.into_iter();
    let value = eval(
        iter_args.next().ok_or(
            DashlangError::new(
                "Expected 'expr' argument",
                ErrorKind::Runtime(errors::RuntimeErrorKind::WrongArgs),
            )
            .location(call.location),
        )?,
        ctx,
    )?;
    println!("{}", stdlib_literal_display(&value, ctx)?);
    Ok(value)
}

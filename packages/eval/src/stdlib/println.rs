use ast::{Call, Literal, Void};
use errors::DashlangError;

use crate::{eval, scope::Scope, Context};

fn stdlib_literal_display<T: Scope + Clone>(
    value: Literal,
    ctx: &Context<T>,
) -> Result<String, DashlangError> {
    match value {
        Literal::Closure(_) => Ok("Closure".to_string()),
        Literal::Int(val) => Ok(format!("{}", val.value)),
        Literal::Float(val) => Ok(format!("{}", val.value)),
        Literal::String(val) => Ok(val.value),
        Literal::Bool(val) => Ok(if val.value {
            "True".to_string()
        } else {
            "False".to_string()
        }),
        Literal::Vector(val) => {
            let display_args: Result<Vec<String>, DashlangError> = val
                .value
                .into_iter()
                .map(|item| stdlib_literal_display(eval(item.clone(), ctx)?, ctx))
                .collect();
            match display_args {
                Ok(args) => Ok(format!("[{}]", args.join(", "))),
                Err(err) => Err(err),
            }
        }
        Literal::Null(_) => Ok("Null".to_string()),
        Literal::Void(_) => Ok("Void".to_string()),
    }
}

pub fn stdlib_println<T: Scope + Clone>(
    call: Call,
    ctx: &Context<T>,
) -> Result<Literal, DashlangError> {
    let mut iter_args = call.args.into_iter();
    let value = eval(iter_args.next().unwrap(), ctx)?;
    println!("{}", stdlib_literal_display(value, ctx)?);
    Ok(Literal::Void(Void {
        location: call.location,
    }))
}

use std::path::Path;

use ast::{Literal, Location, Void};

use crate::{errors::RuntimeResult, eval, scope::Scope, Context};

fn stdlib_literal_display<T: Scope + Clone, P: AsRef<Path> + Clone>(
    value: Literal,
    ctx: &Context<T, P>,
    source_path: P,
) -> RuntimeResult<String> {
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
            let display_args: RuntimeResult<Vec<String>> = val
                .value
                .into_iter()
                .map(|item| {
                    stdlib_literal_display(
                        eval(item.clone(), ctx, source_path.clone())?,
                        ctx,
                        source_path.clone(),
                    )
                })
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

pub fn stdlib_println<T: Scope + Clone, P: AsRef<Path> + Clone>(
    value: Literal,
    ctx: &Context<T, P>,
    source_path: P,
) -> RuntimeResult<Literal> {
    println!("{}", stdlib_literal_display(value, ctx, source_path)?);
    Ok(Literal::Void(Void {
        location: Location::default(),
    }))
}

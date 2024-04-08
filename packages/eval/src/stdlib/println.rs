use ast::Literal;

use crate::{errors::RuntimeResult, eval, scope::Scope, Context};

fn stdlib_literal_display<T: Scope + Clone>(
    value: Literal,
    ctx: &Context<T>,
) -> RuntimeResult<String> {
    match value {
        Literal::Closure(_) => Ok(format!("Closure")),
        Literal::Int(val) => Ok(format!("{val}")),
        Literal::Float(val) => Ok(format!("{val}")),
        Literal::String(val) => Ok(format!("{val}")),
        Literal::Bool(val) => Ok(if val {
            format!("True")
        } else {
            format!("False")
        }),
        Literal::Vector(val) => {
            let display_args: RuntimeResult<Vec<String>> = val
                .into_iter()
                .map(|item| stdlib_literal_display(eval(item.clone(), ctx)?, ctx))
                .collect();
            match display_args {
                Ok(args) => Ok(format!("[{}]", args.join(", "))),
                Err(err) => Err(err),
            }
        }
        Literal::Null => Ok(format!("Null")),
        Literal::Void => Ok(format!("Void")),
    }
}

pub fn stdlib_println<T: Scope + Clone>(
    value: Literal,
    ctx: &Context<T>,
) -> RuntimeResult<Literal> {
    println!("{}", stdlib_literal_display(value, ctx)?);
    Ok(Literal::Void)
}

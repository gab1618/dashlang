use ast::Literal;

use crate::{eval, scope::Scope, Context};

fn stdlib_literal_display<T: Scope + Clone>(value: Literal, ctx: &Context<T>) -> String {
    match value {
        Literal::Closure(_) => format!("Closure"),
        Literal::Int(val) => format!("{val}"),
        Literal::Float(val) => format!("{val}"),
        Literal::String(val) => format!("{val}"),
        Literal::Bool(val) => {
            if val {
                format!("True")
            } else {
                format!("False")
            }
        }
        Literal::Vector(val) => {
            format!(
                "[{}]",
                val.into_iter()
                    .map(|item| stdlib_literal_display(eval(item.clone(), ctx), ctx))
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
        Literal::Null => format!("Null"),
        Literal::Void => format!("Void"),
    }
}

pub fn stdlib_println<T: Scope + Clone>(value: Literal, ctx: &Context<T>) -> Literal {
    println!("{}", stdlib_literal_display(value, ctx));
    Literal::Void
}

use std::rc::Rc;

use ast::Literal;

use crate::{eval, Context, Extension};

use super::{Plugin, Scope};

pub struct Stdlib {}

fn stdlib_literal_display<T: Scope + Clone>(value: &Literal, ctx: &Context<T>) -> String {
    match value {
        Literal::Closure(_) => format!("Closure"),
        Literal::Int(val) => format!("{val}"),
        Literal::Float(val) => format!("{val}"),
        Literal::String(val) => format!("{val}"),
        Literal::Bool(val) => {
            if *val {
                format!("True")
            } else {
                format!("False")
            }
        }
        Literal::Vector(val) => {
            format!(
                "[{}]",
                val.into_iter()
                    .map(|item| stdlib_literal_display(&eval(item.clone(), ctx), ctx))
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
        Literal::Null => format!("Null"),
        Literal::Void => format!("Void"),
    }
}

fn stdlib_println<T: Scope + Clone>(value: &Literal, ctx: &Context<T>) -> Literal {
    println!("{}", stdlib_literal_display(value, ctx));
    Literal::Void
}

impl<T: Scope + Clone> Plugin<T> for Stdlib {
    fn get_extensions(&self) -> Vec<(String, crate::Extension<T>)> {
        vec![(
            String::from("println"),
            Extension {
                params: vec![String::from("expr")],
                implementation: Rc::new(|ctx| {
                    let expr = &ctx.scope.get("expr");
                    stdlib_println(expr, &ctx)
                }),
            },
        )]
    }
}

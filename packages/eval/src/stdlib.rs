use std::rc::Rc;

use ast::Literal;

use crate::Extension;

use super::{Plugin, Scope};

pub struct Stdlib {}

impl<T: Scope> Plugin<T> for Stdlib {
    fn get_extensions(&self) -> Vec<(String, crate::Extension<T>)> {
        vec![(
            String::from("println"),
            Extension {
                params: vec![String::from("expr")],
                implementation: Rc::new(|ctx| {
                    let expr = &ctx.scope.get("expr");
                    match expr {
                        Literal::Closure(_) => println!("Closure"),
                        Literal::Int(val) => println!("{val}"),
                        Literal::Float(val) => println!("{val}"),
                        Literal::String(val) => println!("{val}"),
                        Literal::Bool(val) => {
                            if *val {
                                println!("True")
                            } else {
                                println!("False")
                            }
                        }
                        Literal::Null => println!("Null"),
                        Literal::Void => println!("Void"),
                    }
                    Literal::Void
                }),
            },
        )]
    }
}

mod len;
mod nth;
mod println;
use std::rc::Rc;

use nth::stdlib_nth;
use println::stdlib_println;

use crate::{scope::Scope, stdlib::len::stdlib_len, Extension, Plugin};

pub struct Stdlib {}
impl<T: Scope + Clone> Plugin<T> for Stdlib {
    fn get_extensions(&self) -> Vec<(String, crate::Extension<T>)> {
        vec![
            (
                String::from("println"),
                Extension {
                    params: vec![String::from("expr")],
                    implementation: Rc::new(|ctx| {
                        let expr = ctx.scope.get("expr");
                        stdlib_println(expr, ctx)
                    }),
                },
            ),
            (
                String::from("nth"),
                Extension {
                    params: vec![String::from("value"), String::from("index")],
                    implementation: Rc::new(|ctx| {
                        let value = ctx.scope.get("value");
                        let index = ctx.scope.get("index");
                        stdlib_nth(value, index, &ctx)
                    }),
                },
            ),
            (
                String::from("len"),
                Extension {
                    params: vec![String::from("item")],
                    implementation: Rc::new(|ctx| {
                        let item = ctx.scope.get("item");
                        stdlib_len(item)
                    }),
                },
            ),
        ]
    }
}

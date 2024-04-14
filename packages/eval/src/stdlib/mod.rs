mod input;
mod len;
mod nth;
mod println;
mod push;

use std::{fmt::Debug, path::Path, rc::Rc};

use input::stdlib_input;
use len::stdlib_len;
use nth::stdlib_nth;
use println::stdlib_println;
use push::stdlib_push;

use crate::{scope::Scope, Extension, Plugin};

pub struct Stdlib {}
impl<T: Scope + Clone, P: AsRef<Path> + Clone + Debug> Plugin<T, P> for Stdlib {
    fn get_extensions(&self) -> Vec<(String, crate::Extension<T, P>)> {
        vec![
            (
                String::from("println"),
                Extension {
                    params: vec![String::from("expr")],
                    implementation: Rc::new(|ctx, source_path, _call| {
                        let expr = ctx.scope.get("expr");
                        stdlib_println(expr, ctx, source_path)
                    }),
                },
            ),
            (
                String::from("nth"),
                Extension {
                    params: vec![String::from("value"), String::from("index")],
                    implementation: Rc::new(|ctx, souce_path, call| {
                        let value = ctx.scope.get("value");
                        let index = ctx.scope.get("index");
                        stdlib_nth(value, index, ctx, souce_path, call)
                    }),
                },
            ),
            (
                String::from("len"),
                Extension {
                    params: vec![String::from("item")],
                    implementation: Rc::new(|ctx, source_path, call| {
                        let item = ctx.scope.get("item");
                        stdlib_len(item, source_path, call)
                    }),
                },
            ),
            (
                String::from("push"),
                Extension {
                    params: vec![String::from("item"), String::from("base")],
                    implementation: Rc::new(|ctx, source_path, call| {
                        let item = ctx.scope.get("item");
                        let base = ctx.scope.get("base");
                        stdlib_push(item, base, source_path, call)
                    }),
                },
            ),
            (
                String::from("input"),
                Extension {
                    params: vec![],
                    implementation: Rc::new(|_ctx, source_path, call| {
                        stdlib_input(source_path, call)
                    }),
                },
            ),
        ]
    }
}

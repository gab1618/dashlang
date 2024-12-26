mod input;
mod len;
mod map;
mod nth;
mod push;
mod stdio;

use std::rc::Rc;

use input::stdlib_input;
use len::stdlib_len;
use nth::stdlib_nth;
use push::stdlib_push;
use stdio::println::stdlib_println;

use crate::{
    scope::Scope,
    stdlib::{
        map::{map_get::stdlib_map_get, map_set::stdlib_map_set},
        stdio::print::stdlib_print,
    },
    Extension, Plugin,
};

pub struct Stdlib {}
impl<T: Scope + Clone> Plugin<T> for Stdlib {
    fn get_extensions(&self) -> Vec<(&'static str, crate::Extension<T>)> {
        vec![
            (
                "print",
                Extension {
                    implementation: Rc::new(|ctx, call| stdlib_print(call, ctx)),
                },
            ),
            (
                "println",
                Extension {
                    implementation: Rc::new(|ctx, call| stdlib_println(call, ctx)),
                },
            ),
            (
                "nth",
                Extension {
                    implementation: Rc::new(|ctx, call| stdlib_nth(ctx, call)),
                },
            ),
            (
                "len",
                Extension {
                    implementation: Rc::new(|ctx, call| stdlib_len(ctx, call)),
                },
            ),
            (
                "push",
                Extension {
                    implementation: Rc::new(|ctx, call| stdlib_push(ctx, call)),
                },
            ),
            (
                "input",
                Extension {
                    implementation: Rc::new(|ctx, call| stdlib_input(ctx, call)),
                },
            ),
            (
                "map_set",
                Extension {
                    implementation: Rc::new(|ctx, call| stdlib_map_set(ctx, call)),
                },
            ),
            (
                "map_get",
                Extension {
                    implementation: Rc::new(|ctx, call| stdlib_map_get(ctx, call)),
                },
            ),
        ]
    }
}

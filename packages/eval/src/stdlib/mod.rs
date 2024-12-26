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
    fn get_extensions(&self) -> Vec<(String, crate::Extension<T>)> {
        vec![
            (
                String::from("print"),
                Extension {
                    implementation: Rc::new(|ctx, call| stdlib_print(call, ctx)),
                },
            ),
            (
                String::from("println"),
                Extension {
                    implementation: Rc::new(|ctx, call| stdlib_println(call, ctx)),
                },
            ),
            (
                String::from("nth"),
                Extension {
                    implementation: Rc::new(|ctx, call| stdlib_nth(ctx, call)),
                },
            ),
            (
                String::from("len"),
                Extension {
                    implementation: Rc::new(|ctx, call| stdlib_len(ctx, call)),
                },
            ),
            (
                String::from("push"),
                Extension {
                    implementation: Rc::new(|ctx, call| stdlib_push(ctx, call)),
                },
            ),
            (
                String::from("input"),
                Extension {
                    implementation: Rc::new(|ctx, call| stdlib_input(ctx, call)),
                },
            ),
            (
                String::from("map_set"),
                Extension {
                    implementation: Rc::new(|ctx, call| stdlib_map_set(ctx, call)),
                },
            ),
            (
                String::from("map_get"),
                Extension {
                    implementation: Rc::new(|ctx, call| stdlib_map_get(ctx, call)),
                },
            ),
        ]
    }
}

mod input;
mod len;
mod map;
mod nth;
mod println;
mod push;

use std::rc::Rc;

use input::stdlib_input;
use len::stdlib_len;
use nth::stdlib_nth;
use println::stdlib_println;
use push::stdlib_push;

use crate::{
    scope::Scope,
    stdlib::map::{map_get::stdlib_map_get, map_set::stdlib_map_set},
    Extension, Plugin,
};

pub struct Stdlib {}
impl<T: Scope + Clone> Plugin<T> for Stdlib {
    fn get_extensions(&self) -> Vec<(String, crate::Extension<T>)> {
        vec![
            (
                String::from("println"),
                Extension {
                    params: vec![String::from("expr")],
                    implementation: Rc::new(|ctx, call| stdlib_println(call, ctx)),
                },
            ),
            (
                String::from("nth"),
                Extension {
                    params: vec![String::from("value"), String::from("index")],
                    implementation: Rc::new(|ctx, call| stdlib_nth(ctx, call)),
                },
            ),
            (
                String::from("len"),
                Extension {
                    params: vec![String::from("item")],
                    implementation: Rc::new(|ctx, call| stdlib_len(ctx, call)),
                },
            ),
            (
                String::from("push"),
                Extension {
                    params: vec![String::from("base"), String::from("item")],
                    implementation: Rc::new(|ctx, call| stdlib_push(ctx, call)),
                },
            ),
            (
                String::from("input"),
                Extension {
                    params: vec![],
                    implementation: Rc::new(|ctx, call| stdlib_input(ctx, call)),
                },
            ),
            (
                String::from("map_set"),
                Extension {
                    params: vec!["map".to_owned(), "key".to_owned(), "value".to_owned()],
                    implementation: Rc::new(|ctx, call| stdlib_map_set(ctx, call)),
                },
            ),
            (
                String::from("map_get"),
                Extension {
                    params: vec!["map".to_owned(), "key".to_owned()],
                    implementation: Rc::new(|ctx, call| stdlib_map_get(ctx, call)),
                },
            ),
        ]
    }
}

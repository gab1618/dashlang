use crate::{
    extension::{Extension, Plugin},
    scope::Scope,
};
use std::rc::Rc;

mod literal_display;
pub mod print;
pub mod println;

use print::stdlib_print;
use println::stdlib_println;

pub struct Stdio {}
impl Stdio {
    pub fn new() -> Self {
        Self {}
    }
}

impl<S: Scope + Clone> Plugin<S> for Stdio {
    fn get_extensions(&self) -> Vec<(&'static str, Extension<S>)> {
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
        ]
    }
}

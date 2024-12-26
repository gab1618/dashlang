use std::collections::HashMap;

use ast::{Literal, Program};
use errors::DashlangResult;

use crate::{
    eval_program,
    extension::{Extension, Plugin},
    scope::Scope,
};

pub struct Context<T: Scope> {
    pub scope: T,
    pub extensions: HashMap<String, Extension<T>>,
}
impl<T: Scope + Clone> Context<T> {
    pub fn new(s: T) -> Self {
        Self {
            scope: s,
            extensions: HashMap::new(),
        }
    }
    pub fn use_extension(&mut self, extension: Extension<T>, name: String) {
        self.extensions.insert(name, extension);
    }
    pub fn run_program(&self, program: Program) -> DashlangResult<Literal> {
        eval_program(program, self)
    }
    pub fn use_plugin<P: Plugin<T>>(&mut self, plug: P) {
        for (name, extension) in plug.get_extensions() {
            self.use_extension(extension, name);
        }
    }
}

impl<T: Scope + Clone> Clone for Context<T> {
    fn clone(&self) -> Self {
        Self {
            scope: self.scope.clone(),
            extensions: self.extensions.clone(),
        }
    }
}

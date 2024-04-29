use std::rc::Rc;

use ast::{Call, Literal};
use errors::DashlangResult;

use crate::{scope::Scope, Context};

type ExtensionImplementation<S> = dyn Fn(&Context<S>, Call) -> DashlangResult<Literal>;
#[derive(Clone)]
pub struct Extension<S: Scope> {
    pub implementation: Rc<ExtensionImplementation<S>>,
}
pub trait Plugin<T: Scope> {
    fn get_extensions(&self) -> Vec<(String, Extension<T>)>;
}

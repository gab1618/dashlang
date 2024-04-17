use std::{
    error::Error,
    fmt::{Debug, Display},
};

use ast::Location;

#[derive(Debug, PartialEq, Eq)]
pub enum RuntimeErrorKind {
    Default,
    NonCallable,
    InvalidOperation,
    WrongArgs,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RuntimeError {
    pub message: String,
    pub location: Option<Location>,
    pub kind: RuntimeErrorKind,
}
impl RuntimeError {
    pub fn new(message: &str, kind: RuntimeErrorKind) -> Self {
        Self {
            message: message.to_owned(),
            location: None,
            kind,
        }
    }
    pub fn location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }
}
impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl Error for RuntimeError {}

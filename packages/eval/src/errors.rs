use std::{error::Error, fmt::Display};

use ast::Location;

#[derive(Debug, PartialEq, Eq)]
pub struct RuntimeError {
    message: String,
    location: Option<Location>,
}
impl RuntimeError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
            location: None,
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
pub type RuntimeResult<T> = Result<T, RuntimeError>;

use std::error::Error;

use ast::Location;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum RuntimeErrorKind {
    Default,
    NonCallable,
    InvalidOperation,
    WrongArgs,
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ParsingErrorKind {
    Default,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ErrorKind {
    Runtime(RuntimeErrorKind),
    Parsing(ParsingErrorKind),
}

#[derive(Debug, PartialEq, Eq)]
pub struct DashlangError {
    pub location: Option<Location>,
    pub message: String,
    pub kind: ErrorKind,
}

impl DashlangError {
    pub fn new(message: &str, kind: ErrorKind) -> Self {
        Self {
            location: None,
            message: message.to_owned(),
            kind,
        }
    }
    pub fn location(mut self, location: Location) -> Self {
        self.location = Some(location);
        self
    }
}

impl std::fmt::Display for DashlangError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for DashlangError {}

pub type DashlangResult<T> = Result<T, DashlangError>;

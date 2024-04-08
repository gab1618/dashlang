use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum RuntimeErrorKind {
    NonCallableError,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RuntimeError {
    message: String,
    kind: RuntimeErrorKind,
}
impl RuntimeError {
    pub fn new(message: &str, kind: RuntimeErrorKind) -> Self {
        Self {
            message: message.to_owned(),
            kind,
        }
    }
}
impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl Error for RuntimeError {}
pub type RuntimeResult<T> = Result<T, RuntimeError>;

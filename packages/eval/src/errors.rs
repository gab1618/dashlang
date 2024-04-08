use std::{error::Error, fmt::Display, path::PathBuf};

#[derive(Debug, PartialEq, Eq)]
pub struct ErrorLocation {
    line: usize,
    col: usize,
    filepath: PathBuf,
}
impl Display for ErrorLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.filepath.display(), self.line, self.col)
    }
}
impl ErrorLocation {
    pub fn new(filepath: PathBuf, line: usize, col: usize) -> Self {
        Self {
            filepath,
            line,
            col,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct RuntimeError {
    message: String,
    location: Option<ErrorLocation>,
}
impl RuntimeError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
            location: None,
        }
    }
    pub fn location(mut self, location: ErrorLocation) -> Self {
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

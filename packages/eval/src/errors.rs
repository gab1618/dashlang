use std::{
    error::Error,
    fmt::{Debug, Display},
    path::Path,
};

use ast::Location;

#[derive(Debug, PartialEq, Eq)]
pub struct ErrorLocation<P: AsRef<Path>> {
    location: Location,
    source_path: P,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RuntimeError<P: AsRef<Path> + Debug> {
    message: String,
    location: Option<ErrorLocation<P>>,
}
impl<P: AsRef<Path> + Debug> RuntimeError<P> {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
            location: None,
        }
    }
    pub fn location(mut self, location: Location, source_path: P) -> Self {
        self.location = Some(ErrorLocation {
            location,
            source_path,
        });
        self
    }
}
impl<P: AsRef<Path> + Debug> Display for RuntimeError<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.location {
            None => write!(f, "{}", self.message),
            Some(loc) => write!(f, "{} at {:#?}", self.message, loc.source_path),
        }
    }
}
impl<P: AsRef<Path> + Debug> Error for RuntimeError<P> {}

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
    location: ErrorLocation<P>,
}
impl<P: AsRef<Path> + Debug> RuntimeError<P> {
    pub fn new(message: &str, location: Location, source_path: P) -> Self {
        Self {
            message: message.to_owned(),
            location: ErrorLocation {
                location,
                source_path,
            },
        }
    }
}
impl<P: AsRef<Path> + Debug> Display for RuntimeError<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} at from {} to {}",
            self.message, self.location.location.start, self.location.location.end
        )
    }
}
impl<P: AsRef<Path> + Debug> Error for RuntimeError<P> {}

use std::error::Error;

#[derive(Debug, Eq, PartialEq)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParsingErrorKind {}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsingError {
    pub message: String,
    pub kind: ParsingErrorKind,
    pub location: Location,
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ParsingError {}

pub type ParsingResult<T> = Result<T, ParsingError>;

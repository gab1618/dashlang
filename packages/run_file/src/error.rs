use std::fmt::{Debug, Display};

use errors::{DashlangError, ErrorKind, RuntimeErrorKind};
use miette::{Diagnostic, LabeledSpan, NamedSource, Result};

use thiserror::Error;
#[derive(Error, Debug)]
pub struct RunfileError {
    pub src: NamedSource<String>,
    pub err: DashlangError,
}

impl Display for RunfileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err.message)
    }
}

impl Diagnostic for RunfileError {
    fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        match self.err.kind {
            ErrorKind::Runtime(runtime_err) => {
                match runtime_err {
                    RuntimeErrorKind::Default => None,
                    RuntimeErrorKind::NonCallable => {
                        Some(Box::new("Check if this value really exists".to_owned()))
                    }
                    RuntimeErrorKind::InvalidOperation => {
                        Some (
                            Box::new(
                                "Try changing the values in the operation. Remember sometimes the order of the operands change the result".to_owned()
                            )
                        )
                    }
                    RuntimeErrorKind::WrongArgs => Some(Box::new("Try fixing the number of arguments passed to this call".to_owned())),
                }
            }
            ErrorKind::Parsing(_) => None 

        }
    }

    fn source_code(&self) -> Option<&dyn miette::SourceCode> {
        Some(&self.src)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        match self.err.location {
            None => None,
            Some(loc) => Some(Box::new(
                [LabeledSpan::at(
                    loc.start..loc.end,
                    match self.err.kind {
                        ErrorKind::Runtime(runtime_err) => match runtime_err {
                            RuntimeErrorKind::Default => "The error is here",
                            RuntimeErrorKind::NonCallable => "Non-callable value here",
                            RuntimeErrorKind::InvalidOperation => "Invalid operation here",
                            RuntimeErrorKind::WrongArgs => "Wrong args passed here",
                        },
                        ErrorKind::Parsing(parsing_err) => match parsing_err {
                            errors::ParsingErrorKind::Default => "Invalid syntax here",
                        },
                    },
                )]
                .into_iter(),
            )),
        }
    }
}

pub type RunfileResult = Result<()>;

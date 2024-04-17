use std::{
    fmt::{Debug, Display},
    fs::read_to_string,
};

use eval::{
    errors::{RuntimeError, RuntimeErrorKind},
    scope::HashScope,
    stdlib::Stdlib,
    Context,
};
use miette::{Diagnostic, LabeledSpan, NamedSource, Result};
use parse::parse;

use thiserror::Error;

#[derive(Error, Debug)]
pub struct RunfileError {
    src: NamedSource<String>,
    err: RuntimeError,
}

impl Display for RunfileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err.message)
    }
}

impl Diagnostic for RunfileError {
    fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        match self.err.kind {
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
            RuntimeErrorKind::WrongArgs => Some(Box::new("Try fixing the number os arguments passed to this call".to_owned())),

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
                        RuntimeErrorKind::Default => "The error is here",
                        RuntimeErrorKind::NonCallable => "Non existent callable here",
                        RuntimeErrorKind::InvalidOperation => "Invalid operation here",
                        RuntimeErrorKind::WrongArgs => "Wrong number os arguments provided",
                    },
                )]
                .into_iter(),
            )),
        }
    }
}

pub type RunfileResult = Result<()>;

pub fn run_file(file_path: &str) -> RunfileResult {
    let scope = HashScope::default();
    let file_content = read_to_string(file_path).unwrap();
    let mut ctx = Context::new(scope);
    ctx.use_plugin(&Stdlib {});
    let program = parse(&file_content);
    match ctx.run_program(program) {
        Ok(_) => Ok(()),
        Err(err) => Err(RunfileError {
            src: NamedSource::new(file_path, file_content),
            err,
        }
        .into()),
    }
}

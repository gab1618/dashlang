pub mod error;

#[cfg(test)]
mod examples_tests;

use parse::parse;
use std::fs::read_to_string;

use error::{RunfileError, RunfileResult};
use eval::{ctx::Context, scope::HashScope, stdlib::Stdlib};
use miette::NamedSource;

pub fn run_file(file_path: &str) -> RunfileResult {
    let scope = HashScope::default();
    let file_content = read_to_string(file_path).unwrap();
    let mut ctx = Context::new(scope);
    ctx.use_plugin(&Stdlib {});
    match parse(&file_content) {
        Err(err) => Err(RunfileError {
            src: NamedSource::new(file_path, file_content),
            err,
        }
        .into()),
        Ok(program) => match ctx.run_program(program) {
            Ok(_) => Ok(()),
            Err(err) => Err(RunfileError {
                src: NamedSource::new(file_path, file_content),
                err,
            }
            .into()),
        },
    }
}

pub mod error;

#[cfg(test)]
mod examples_tests;

use parse::parse;
use std::fs::read_to_string;

use error::{RunfileError, RunfileResult};
use eval::{ctx::Context, scope::Scope};
use miette::NamedSource;

pub fn run_file<T: Scope + Clone>(file_path: &str, ctx: &mut Context<T>) -> RunfileResult {
    let file_content = read_to_string(file_path).unwrap();
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

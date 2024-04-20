use ast::Program;
use errors::DashlangResult;
use file::parse_file;
use parser::{DashlangParser, Rule};

mod body;
mod expression;
mod file;
mod literal;
mod parser;
mod program;
mod statement;
mod utils;

#[cfg(test)]
mod examples_tests;

pub fn parse(input: &str) -> DashlangResult<Program> {
    parse_file(input)
}

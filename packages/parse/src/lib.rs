use ast::Program;
use errors::DashlangResult;
use parser::{DashlangParser, Rule};

mod body;
mod expression;
mod literal;
mod parser;
mod program;
mod statement;
mod utils;

use program::parse_program;

pub fn parse(input: &str) -> DashlangResult<Program> {
    parse_program(input)
}

use ast::Program;
use errors::ParsingResult;
use parser::{DashlangParser, Rule};

mod body;
mod errors;
mod expression;
mod instruction;
mod literal;
mod parser;
mod program;
mod statement;
mod utils;

use program::parse_program;

pub fn parse(input: &str) -> ParsingResult<Program> {
    parse_program(input)
}

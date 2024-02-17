use ast::Program;
use parser::{DashlangParser, Rule};
use values::parse_values;

mod expression;
mod instruction;
mod parser;
mod program;
mod statement;
mod values;

use program::parse_program;

pub fn parse(input: &str) -> Program {
    parse_program(input)
}

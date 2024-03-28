use ast::Program;
use parser::{DashlangParser, Rule};

#[cfg(test)]
mod examples_tests;
mod expression;
mod instruction;
mod parser;
mod program;
mod scope;
mod statement;
mod value;

use program::parse_program;

pub fn parse(input: &str) -> Program {
    parse_program(input)
}

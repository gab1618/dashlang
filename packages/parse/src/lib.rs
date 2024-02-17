use ast::Program;
use parser::{DashlangParser, Rule};
use values::parse_values;

mod expression;
mod instruction;
mod parser;
mod statement;
mod values;

pub fn parse(input: &str) -> Program {
    todo!()
}

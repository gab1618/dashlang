use ast::Program;
use pest::Parser;

use crate::{
    instruction::parse_instruction,
    parser::{DashlangParser, Rule},
};

pub fn parse_scope(input: &str) -> Program {
    let mut body: Program = vec![];
    let ast = DashlangParser::parse(Rule::scope, input)
        .expect("Could not parse scope")
        .next()
        .expect("Could not parse scope")
        .into_inner()
        .next()
        .expect("Could not parse program");
    for instruction in ast.into_inner() {
        let parsed_instruction = parse_instruction(instruction.as_str());
        body.push(parsed_instruction);
    }
    body
}

use ast::Program;
use pest::Parser;

use crate::{
    errors::ParsingResult,
    instruction::parse_instruction,
    parser::{DashlangParser, Rule},
    utils::get_pair_location,
};

pub fn parse_body(input: &str, base_location: usize) -> ParsingResult<Program> {
    let mut body: Program = vec![];
    let ast = DashlangParser::parse(Rule::body, input)
        .expect("Could not parse scope")
        .next()
        .expect("Could not parse scope")
        .into_inner()
        .next()
        .expect("Could not parse program");
    for instruction in ast.into_inner() {
        let (start, _end) = get_pair_location(&instruction);
        let parsed_instruction = parse_instruction(instruction.as_str(), start + base_location)?;
        body.push(parsed_instruction);
    }
    Ok(body)
}

use ast::Program;
use errors::DashlangResult;
use pest::Parser;

use crate::{
    parser::{DashlangParser, Rule},
    statement::parse_statement,
    utils::get_pair_location,
};

pub fn parse_body(input: &str, base_location: usize) -> DashlangResult<Program> {
    let mut body: Program = vec![];
    let ast = DashlangParser::parse(Rule::body, input)
        .expect("Could not parse scope")
        .next()
        .expect("Could not parse scope")
        .into_inner()
        .next()
        .expect("Could not parse program");
    for stmt in ast.into_inner() {
        let (start, _end) = get_pair_location(&stmt);
        let parsed_stmt = parse_statement(stmt.as_str(), start + base_location)?;
        body.push(parsed_stmt);
    }
    Ok(body)
}

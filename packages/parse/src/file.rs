use ast::{Location, Program};
use errors::{DashlangError, DashlangResult, ErrorKind, ParsingErrorKind};
use pest::Parser;

use crate::{
    parser::{DashlangParser, Rule},
    program::parse_program,
};

pub fn parse_file(input: &str) -> DashlangResult<Program> {
    let ast = DashlangParser::parse(Rule::file, input)
        .map_err(|err| DashlangError {
            location: match err.location {
                pest::error::InputLocation::Pos(pos) => Some(Location::new(pos, pos + 1)),
                pest::error::InputLocation::Span((start, end)) => Some(Location::new(start, end)),
            },
            message: err.to_string(),
            kind: ErrorKind::Parsing(ParsingErrorKind::Default),
        })?
        .next()
        .expect("Could not parse program");
    let ast_program = ast.into_inner().next().expect("Could not parse file");
    let parsed = parse_program(ast_program.as_str())?;
    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_hello_world() {
        parse_file("println(true)").unwrap();
    }
    #[test]
    #[should_panic]
    fn test_invalid_assignment() {
        parse_file("a = ").unwrap();
    }
}

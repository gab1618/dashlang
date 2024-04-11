use ast::Program;
use pest::Parser;

use crate::{
    instruction::parse_instruction,
    parser::{DashlangParser, Rule},
};

pub fn parse_program(input: &str) -> Program {
    let mut program: Program = vec![];
    let ast = DashlangParser::parse(Rule::program, input)
        .expect("Could not parse program")
        .next()
        .expect("Could not parse program");
    for instruction in ast.into_inner() {
        program.push(parse_instruction(instruction.as_str()));
    }
    program
}

#[cfg(test)]
mod tests {

    use ast::{AssignmentExpr, Expr, Instruction, Int, Literal, Location};

    use super::*;

    #[test]
    fn test_parse_program() {
        assert_eq!(
            parse_program("age = 5 count = 1"),
            vec![
                Instruction::Expr(Expr::Assignment(AssignmentExpr {
                    symbol: String::from("age"),
                    value: Box::new(Expr::Literal(Literal::Int(Int {
                        value: 5,
                        location: Default::default()
                    }))),
                    location: Location::default(),
                })),
                Instruction::Expr(Expr::Assignment(AssignmentExpr {
                    symbol: String::from("count"),
                    value: Box::new(Expr::Literal(Literal::Int(Int {
                        value: 1,
                        location: Default::default()
                    }))),
                    location: Location::default(),
                }))
            ]
        )
    }
}

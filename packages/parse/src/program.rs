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

    use ast::{Asignment, Expr, Instruction, Literal};

    use super::*;

    #[test]
    fn test_parse_program() {
        assert_eq!(
            parse_program("age = 5 count = 1"),
            vec![
                Instruction::Expr(Expr::Asignment(Asignment {
                    symbol: String::from("age"),
                    value: Box::new(Expr::Literal(Literal::Int(5)))
                })),
                Instruction::Expr(Expr::Asignment(Asignment {
                    symbol: String::from("count"),
                    value: Box::new(Expr::Literal(Literal::Int(1)))
                }))
            ]
        )
    }
}

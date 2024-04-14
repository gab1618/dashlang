use ast::Instruction;
use pest::Parser;

use crate::{
    expression::parse_expression,
    parser::{DashlangParser, Rule},
    statement::parse_statement,
    utils::get_pair_location,
};

pub fn parse_instruction(input: &str, base_location: usize) -> Instruction {
    let ast = DashlangParser::parse(Rule::instruction, input)
        .expect("Could not parse instruction")
        .next()
        .expect("Could not parse instruction");
    let (start, _end) = get_pair_location(&ast);
    let instruction_type = ast
        .into_inner()
        .next()
        .expect("Could not get instruction type");
    match instruction_type.as_rule() {
        Rule::statement => Instruction::Stmt(parse_statement(
            instruction_type.as_str(),
            start + base_location,
        )),
        Rule::expression => Instruction::Expr(parse_expression(
            instruction_type.as_str(),
            start + base_location,
        )),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use ast::{BinaryExpr, BinaryOperator, Expr, Int, Literal, Location, Return, Stmt};

    use super::*;
    #[test]
    fn test_parse_expr() {
        assert_eq!(
            parse_instruction("1 + 1", 0),
            Instruction::Expr(Expr::BinaryExpr(Box::new(BinaryExpr {
                left: Expr::Literal(Literal::Int(Int {
                    value: 1,
                    location: Location::new(0, 1)
                })),
                right: Expr::Literal(Literal::Int(Int {
                    value: 1,
                    location: Location::new(4, 5)
                })),
                operator: BinaryOperator::Add,
                location: Location::new(0, 5),
            })))
        );
    }
    #[test]
    fn test_parse_stmt() {
        assert_eq!(
            parse_instruction("return 1", 0),
            Instruction::Stmt(Stmt::Return(Return {
                value: Expr::Literal(Literal::Int(Int {
                    value: 1,
                    location: Location::new(7, 8)
                })),
                location: Location::new(0, 8)
            }))
        );
    }
}

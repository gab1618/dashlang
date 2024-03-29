use ast::Instruction;
use pest::Parser;

use crate::{
    expression::parse_expression,
    parser::{DashlangParser, Rule},
    statement::parse_statement,
};

pub fn parse_instruction(input: &str) -> Instruction {
    let ast = DashlangParser::parse(Rule::instruction, input)
        .expect("Could not parse instruction")
        .next()
        .expect("Could not parse instruction");
    let instruction_type = ast
        .into_inner()
        .next()
        .expect("Could not get instruction type");
    match instruction_type.as_rule() {
        Rule::statement => Instruction::Stmt(parse_statement(instruction_type.as_str())),
        Rule::expression => Instruction::Expr(parse_expression(instruction_type.as_str())),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use ast::{BinaryExpr, BinaryOperator, Expr, Literal, Stmt};

    use super::*;
    #[test]
    fn test_parse_expr() {
        assert_eq!(
            parse_instruction("1 + 1"),
            Instruction::Expr(Expr::BinaryExpr(Box::new(BinaryExpr {
                left: Expr::Literal(Literal::Int(1)),
                right: Expr::Literal(Literal::Int(1)),
                operator: BinaryOperator::Add
            })))
        );
    }
    #[test]
    fn test_parse_stmt() {
        assert_eq!(
            parse_instruction("return 1"),
            Instruction::Stmt(Stmt::Return(Expr::Literal(Literal::Int(1))))
        );
    }
}

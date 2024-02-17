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
    use ast::{BinaryOp, BinaryOpType, Expr, Stmt, Value};

    use super::*;
    #[test]
    fn test_parse_expr() {
        assert_eq!(
            parse_instruction("1 + 1"),
            Instruction::Expr(Expr::BinaryOp(Box::new(BinaryOp {
                left: Expr::Value(Value::Int(1)),
                right: Expr::Value(Value::Int(1)),
                op_type: BinaryOpType::Add
            })))
        );
    }
    #[test]
    fn test_parse_stmt() {
        assert_eq!(
            parse_instruction("return 1"),
            Instruction::Stmt(Stmt::Return(Expr::Value(Value::Int(1))))
        );
    }
}

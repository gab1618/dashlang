use ast::{Expr, If, Value};
use pest::Parser;

use crate::{
    expression::parse_expression,
    parser::{DashlangParser, Rule},
    scope::parse_scope,
    value::parse_values,
};

pub fn parse_if_stmt(input: &str) -> If {
    let mut final_if = If {
        cond: Expr::Value(Value::Bool(true)),
        body: vec![],
        else_block: None,
    };
    let ast = DashlangParser::parse(Rule::if_stmt, input)
        .expect("Could not parse if statement")
        .next()
        .expect("Could not parse if statement");
    for element in ast.into_inner() {
        match element.as_rule() {
            Rule::expression => {
                final_if.cond = parse_expression(element.as_str());
            }
            Rule::value => {
                final_if.cond = Expr::Value(parse_values(element.as_str()));
            }
            Rule::scope => {
                final_if.body = parse_scope(element.as_str());
            }
            _ => unreachable!(),
        }
    }
    final_if
}
#[cfg(test)]
mod tests {

    use ast::{BinaryOp, BinaryOpType, Expr, Instruction, Stmt, Value};

    use super::*;

    #[test]
    fn test_if_with_values() {
        assert_eq!(
            parse_if_stmt("if true {}"),
            If {
                cond: Expr::Value(Value::Bool(true)),
                body: vec![],
                else_block: None
            }
        );
    }
    #[test]
    fn test_if_with_symbols() {
        assert_eq!(
            parse_if_stmt("if count < 10 {}"),
            If {
                cond: Expr::BinaryOp(Box::new(BinaryOp {
                    left: Expr::Symbol(String::from("count")),
                    right: Expr::Value(Value::Int(10)),
                    op_type: BinaryOpType::Lt
                })),
                body: vec![],
                else_block: None
            }
        );
        assert_eq!(
            parse_if_stmt("if count < 10 {return true}"),
            If {
                cond: Expr::BinaryOp(Box::new(BinaryOp {
                    left: Expr::Symbol(String::from("count")),
                    right: Expr::Value(Value::Int(10)),
                    op_type: BinaryOpType::Lt
                })),
                body: vec![Instruction::Stmt(Stmt::Return(Expr::Value(Value::Bool(
                    true
                ))))],
                else_block: None
            }
        );
    }
}

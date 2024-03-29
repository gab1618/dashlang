use ast::{Expr, If, Literal};
use pest::Parser;

use crate::{
    expression::parse_expression,
    literal::parse_literal,
    parser::{DashlangParser, Rule},
    scope::parse_scope,
};

pub fn parse_if_stmt(input: &str) -> If {
    let mut final_if = If {
        cond: Expr::Literal(Literal::Bool(true)),
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
            Rule::literal => {
                final_if.cond = Expr::Literal(parse_literal(element.as_str()));
            }
            Rule::scope => {
                final_if.body = parse_scope(element.as_str());
            }
            Rule::else_stmt => {
                let else_block_ast = element
                    .into_inner()
                    .next()
                    .expect("Could not get scope from else block");
                final_if.else_block = Some(parse_scope(else_block_ast.as_str()));
            }
            _ => unreachable!(),
        }
    }
    final_if
}
#[cfg(test)]
mod tests {

    use ast::{BinaryOp, BinaryOpType, Expr, Instruction, Literal, Stmt};

    use super::*;

    #[test]
    fn test_if_with_values() {
        assert_eq!(
            parse_if_stmt("if true {}"),
            If {
                cond: Expr::Literal(Literal::Bool(true)),
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
                    right: Expr::Literal(Literal::Int(10)),
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
                    right: Expr::Literal(Literal::Int(10)),
                    op_type: BinaryOpType::Lt
                })),
                body: vec![Instruction::Stmt(Stmt::Return(Expr::Literal(
                    Literal::Bool(true)
                )))],
                else_block: None
            }
        );
    }
    #[test]
    fn test_else() {
        assert_eq!(
            parse_if_stmt("if true {return true} else {return false}"),
            If {
                cond: Expr::Literal(Literal::Bool(true)),
                body: vec![Instruction::Stmt(Stmt::Return(Expr::Literal(
                    Literal::Bool(true)
                )))],
                else_block: Some(vec![Instruction::Stmt(Stmt::Return(Expr::Literal(
                    Literal::Bool(false)
                )))])
            }
        );
    }
}

use ast::{Expr, Literal, Location, While};
use pest::Parser;

use crate::{
    body::parse_body,
    expression::parse_expression,
    literal::parse_literal,
    parser::{DashlangParser, Rule},
};

pub fn parse_while_stmt(input: &str) -> While {
    let mut final_while = While {
        cond: Expr::Literal(Literal::Bool(true)),
        body: vec![],
        location: Location::default(),
    };
    let ast = DashlangParser::parse(Rule::while_stmt, input)
        .expect("Could not parse while loop")
        .next()
        .unwrap();
    for element in ast.into_inner() {
        match element.as_rule() {
            Rule::literal => {
                final_while.cond = Expr::Literal(parse_literal(element.as_str()));
            }
            Rule::expression => {
                final_while.cond = parse_expression(element.as_str());
            }
            Rule::body => {
                final_while.body = parse_body(element.as_str());
            }
            _ => unreachable!(),
        }
    }
    final_while
}
#[cfg(test)]
mod tests {
    use ast::{BinaryExpr, BinaryOperator, Instruction, Stmt};

    use super::*;

    #[test]
    fn test_while_with_values() {
        assert_eq!(
            parse_while_stmt("while true {}"),
            While {
                cond: Expr::Literal(Literal::Bool(true)),
                body: vec![],
                location: Location::default(),
            }
        );
    }
    #[test]
    fn test_parse_while() {
        assert_eq!(
            parse_while_stmt("while count < 10 {}"),
            While {
                cond: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Symbol(String::from("count")),
                    right: Expr::Literal(Literal::Int(10)),
                    operator: BinaryOperator::Lt,
                    location: Location::default(),
                })),
                body: vec![],
                location: Location::default(),
            }
        );

        assert_eq!(
            parse_while_stmt("while count < 10 {return 1}"),
            While {
                cond: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Symbol(String::from("count")),
                    right: Expr::Literal(Literal::Int(10)),
                    operator: BinaryOperator::Lt,
                    location: Location::default(),
                })),
                body: vec![Instruction::Stmt(Stmt::Return(Expr::Literal(
                    Literal::Int(1)
                )))],
                location: Location::default(),
            }
        );
    }
}

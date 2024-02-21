use crate::{DashlangParser, Rule};
use ast::Expr;
use pest::Parser;

use self::{
    asignment_expression::parse_asignment_expression, binary_expression::parse_binary_expression,
    call_expression::parse_call_expression,
};

mod asignment_expression;
mod binary_expression;
mod call_expression;

pub fn parse_expression(input: &str) -> Expr {
    let ast = DashlangParser::parse(Rule::expression, input)
        .expect("Could not parse expression")
        .next()
        .expect("Could not parse expression");
    let expression = ast
        .into_inner()
        .next()
        .expect("Could not get expression type");
    match expression.as_rule() {
        Rule::binary_expression => {
            let parsed = parse_binary_expression(expression.as_str());
            Expr::BinaryOp(Box::new(parsed))
        }
        Rule::asignment_expression => {
            let parsed = parse_asignment_expression(expression.as_str());
            Expr::Asignment(parsed)
        }
        Rule::call_expression => {
            let parsed = parse_call_expression(expression.as_str());
            Expr::Call(parsed)
        }
        Rule::symbol => Expr::Symbol(expression.as_str().to_owned()),
        _ => unreachable!(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use ast::{Asignment, BinaryOp, BinaryOpType, Expr, Value};
    #[test]
    fn test_parse_expression() {
        assert_eq!(
            parse_expression("1 + 2"),
            Expr::BinaryOp(Box::new(BinaryOp {
                left: Expr::Value(Value::Int(1)),
                right: Expr::Value(Value::Int(2)),
                op_type: BinaryOpType::Add
            }))
        );
    }
    #[test]
    fn test_asignment_expression() {
        assert_eq!(
            parse_expression("age = 5 + 1"),
            Expr::Asignment(Asignment {
                symbol: String::from("age"),
                value: Box::new(Expr::BinaryOp(Box::new(BinaryOp {
                    left: Expr::Value(Value::Int(5)),
                    right: Expr::Value(Value::Int(1)),
                    op_type: BinaryOpType::Add
                })))
            })
        );
    }
}

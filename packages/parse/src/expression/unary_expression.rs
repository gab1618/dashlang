use ast::{Expr, Literal, UnaryOp, UnaryOpType};
use pest::Parser;

use crate::{
    literal::parse_literal,
    parser::{DashlangParser, Rule},
};

use super::parse_expression;

pub fn parse_unary_expression(input: &str) -> UnaryOp {
    let parsed = DashlangParser::parse(Rule::unary_expression, input)
        .expect("Could not parse unary expression")
        .next()
        .expect("Could not get unary expression");
    let mut parsed_inner = parsed.into_inner();
    let operator = parsed_inner
        .next()
        .expect("Could not get unary expression operator");
    let operand = parsed_inner
        .next()
        .expect("Could not get unary expression operand");
    UnaryOp {
        op_type: match operator.as_str() {
            "!" => UnaryOpType::Not,
            any => panic!("Invalid unary operator: {any}"),
        },
        operand: Expr::Literal(parse_literal(operand.as_str())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_true() {
        assert_eq!(
            parse_unary_expression("!true"),
            UnaryOp {
                op_type: ast::UnaryOpType::Not,
                operand: Expr::Literal(Literal::Bool(true))
            }
        );
    }
}

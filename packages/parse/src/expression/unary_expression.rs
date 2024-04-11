use ast::{Location, UnaryExpr, UnaryOperator};
use pest::Parser;

use crate::parser::{DashlangParser, Rule};

use super::parse_expression;

pub fn parse_unary_expression(input: &str) -> UnaryExpr {
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
    UnaryExpr {
        operator: match operator.as_str() {
            "!" => UnaryOperator::Not,
            any => panic!("Invalid unary operator: {any}"),
        },
        operand: (parse_expression(operand.as_str())),
        location: Location::default(),
    }
}

#[cfg(test)]
mod tests {
    use ast::{BinaryExpr, BinaryOperator, Boolean, Expr, Literal};

    use super::*;

    #[test]
    fn test_not_true() {
        assert_eq!(
            parse_unary_expression("!true"),
            UnaryExpr {
                operator: ast::UnaryOperator::Not,
                operand: Expr::Literal(Literal::Bool(Boolean {
                    value: true,
                    location: Default::default()
                })),
                location: Location::default(),
            }
        );
    }
    #[test]
    fn test_sub() {
        assert_eq!(
            parse_unary_expression("!(true && false)"),
            UnaryExpr {
                operator: UnaryOperator::Not,
                operand: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Literal(Literal::Bool(Boolean {
                        value: true,
                        location: Default::default()
                    })),
                    right: Expr::Literal(Literal::Bool(Boolean {
                        value: false,
                        location: Default::default()
                    })),
                    operator: BinaryOperator::And,
                    location: Location::default(),
                })),
                location: Location::default(),
            }
        );
    }
}

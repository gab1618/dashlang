use ast::{Location, UnaryExpr, UnaryOperator};
use pest::Parser;

use crate::{
    errors::ParsingResult,
    parser::{DashlangParser, Rule},
    utils::get_pair_location,
};

use super::parse_expression;

pub fn parse_unary_expression(input: &str, base_location: usize) -> ParsingResult<UnaryExpr> {
    let parsed = DashlangParser::parse(Rule::unary_expression, input)
        .expect("Could not parse unary expression")
        .next()
        .expect("Could not get unary expression");
    let (start, end) = get_pair_location(&parsed);
    let mut parsed_inner = parsed.into_inner();
    let operator = parsed_inner
        .next()
        .expect("Could not get unary expression operator");
    let operand = parsed_inner
        .next()
        .expect("Could not get unary expression operand");
    let (operand_start, _) = get_pair_location(&operand);
    Ok(UnaryExpr {
        operator: match operator.as_str() {
            "!" => UnaryOperator::Not,
            any => panic!("Invalid unary operator: {any}"),
        },
        operand: (parse_expression(operand.as_str(), operand_start + base_location)?),
        location: Location::new(start, end),
    })
}

#[cfg(test)]
mod tests {
    use ast::{BinaryExpr, BinaryOperator, Boolean, Expr, Literal};

    use super::*;

    #[test]
    fn test_not_true() {
        assert_eq!(
            parse_unary_expression("!true", 0),
            Ok(UnaryExpr {
                operator: ast::UnaryOperator::Not,
                operand: Expr::Literal(Literal::Bool(Boolean {
                    value: true,
                    location: Location::new(1, 5)
                })),
                location: Location::new(0, 5),
            })
        );
    }
    #[test]
    fn test_sub() {
        assert_eq!(
            parse_unary_expression("!(true && false)", 0),
            Ok(UnaryExpr {
                operator: UnaryOperator::Not,
                operand: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Literal(Literal::Bool(Boolean {
                        value: true,
                        location: Location::new(2, 6)
                    })),
                    right: Expr::Literal(Literal::Bool(Boolean {
                        value: false,
                        location: Location::new(10, 15)
                    })),
                    operator: BinaryOperator::And,
                    location: Location::new(2, 15),
                })),
                location: Location::new(0, 16),
            })
        );
    }
}

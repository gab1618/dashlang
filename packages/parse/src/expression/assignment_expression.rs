use pest::Parser;

use crate::{expression::parse_expression, utils::get_pair_location, DashlangParser, Rule};
use ast::{AssignmentExpr, Location};

pub fn parse_assignment_expression(input: &str) -> AssignmentExpr {
    let ast = DashlangParser::parse(Rule::assignment_expression, input)
        .expect("Could not parse assignment expression")
        .next()
        .expect("Could not parse assignment expression");
    let (start, end) = get_pair_location(&ast);
    let mut ast_inner = ast.into_inner();
    let ast_symbol = ast_inner
        .next()
        .expect("Could not get assignment expression symbol");
    let ast_value = ast_inner
        .next()
        .expect("Could not get assignment expression value");
    AssignmentExpr {
        symbol: ast_symbol.as_str().to_owned(),
        value: Box::new(parse_expression(ast_value.as_str())),
        location: Location::new(start, end),
    }
}

#[cfg(test)]
mod tests {
    use ast::{BinaryExpr, BinaryOperator, Expr, Int, Literal};

    use super::*;
    #[test]
    fn test_parse_value_assignment() {
        assert_eq!(
            parse_assignment_expression("age = 5"),
            AssignmentExpr {
                symbol: String::from("age"),
                value: Box::new(Expr::Literal(Literal::Int(Int {
                    value: 5,
                    location: Location::new(0, 1)
                }))),
                location: Location::new(0, 7),
            }
        );
    }
    #[test]
    fn test_parse_expr_assignment() {
        assert_eq!(
            parse_assignment_expression("age = 5 + 1"),
            AssignmentExpr {
                symbol: String::from("age"),
                value: Box::new(Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Literal(Literal::Int(Int {
                        value: 5,
                        location: Location::new(0, 1)
                    })),
                    right: Expr::Literal(Literal::Int(Int {
                        value: 1,
                        location: Location::new(0, 1)
                    })),
                    operator: BinaryOperator::Add,
                    location: Location::default(),
                }))),
                location: Location::new(0, 11),
            }
        );
    }
}

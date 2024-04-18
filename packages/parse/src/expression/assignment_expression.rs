use errors::DashlangResult;
use pest::Parser;

use crate::{expression::parse_expression, utils::get_pair_location, DashlangParser, Rule};
use ast::{AssignmentExpr, Location};

pub fn parse_assignment_expression(
    input: &str,
    base_location: usize,
) -> DashlangResult<AssignmentExpr> {
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
    let (start_value, _) = get_pair_location(&ast_value);
    Ok(AssignmentExpr {
        symbol: ast_symbol.as_str().to_owned(),
        value: Box::new(parse_expression(
            ast_value.as_str(),
            start_value + base_location,
        )?),
        location: Location::new(start + base_location, end + base_location),
    })
}

#[cfg(test)]
mod tests {
    use ast::{BinaryExpr, BinaryOperator, Expr, Int, Literal};

    use super::*;
    #[test]
    fn test_parse_value_assignment() {
        assert_eq!(
            parse_assignment_expression("age = 5", 0),
            Ok(AssignmentExpr {
                symbol: String::from("age"),
                value: Box::new(Expr::Literal(Literal::Int(Int {
                    value: 5,
                    location: Location::new(6, 7)
                }))),
                location: Location::new(0, 7),
            })
        );
    }
    #[test]
    fn test_parse_expr_assignment() {
        assert_eq!(
            parse_assignment_expression("age = 5 + 1", 0),
            Ok(AssignmentExpr {
                symbol: String::from("age"),
                value: Box::new(Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Literal(Literal::Int(Int {
                        value: 5,
                        location: Location::new(6, 7)
                    })),
                    right: Expr::Literal(Literal::Int(Int {
                        value: 1,
                        location: Location::new(10, 11)
                    })),
                    operator: BinaryOperator::Add,
                    location: Location::new(6, 11),
                }))),
                location: Location::new(0, 11),
            })
        );
    }
}

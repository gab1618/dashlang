use ast::{Expr, Location, Return, Stmt};
use pest::Parser;

use crate::{
    expression::parse_expression,
    literal::parse_literal,
    parser::{DashlangParser, Rule},
    utils::get_pair_location,
};

pub fn parse_return_stmt(input: &str, base_location: usize) -> Stmt {
    let ast = DashlangParser::parse(Rule::return_stmt, input)
        .expect("Could not parse return statement")
        .next()
        .expect("Could not parse return statement");
    let (start, end) = get_pair_location(&ast);
    let return_stmt = ast
        .into_inner()
        .next()
        .expect("Could not get return statement");
    let (ret_start, _) = get_pair_location(&return_stmt);
    let return_value = match return_stmt.as_rule() {
        Rule::literal => {
            let value = return_stmt
                .into_inner()
                .next()
                .expect("Could not get value");
            Expr::Literal(parse_literal(value.as_str(), ret_start + base_location))
        }
        Rule::expression => parse_expression(return_stmt.as_str(), ret_start + base_location),
        _ => unreachable!(),
    };
    Stmt::Return(Return {
        value: return_value,
        location: Location::new(start + base_location, end + base_location),
    })
}

#[cfg(test)]
mod tests {
    use ast::{BinaryExpr, BinaryOperator, Expr, Int, Literal, Location, Return};

    use super::*;
    #[test]
    fn test_return_value() {
        assert_eq!(
            parse_return_stmt("return 1", 0),
            Stmt::Return(Return {
                value: Expr::Literal(Literal::Int(Int {
                    value: 1,
                    location: Location::new(7, 8)
                })),
                location: Location::new(0, 8)
            })
        );
    }
    #[test]
    fn test_return_expression() {
        assert_eq!(
            parse_return_stmt("return 1 + 1", 0),
            Stmt::Return(Return {
                value: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Literal(Literal::Int(Int {
                        value: 1,
                        location: Location::new(7, 8)
                    })),
                    right: Expr::Literal(Literal::Int(Int {
                        value: 1,
                        location: Location::new(11, 12)
                    })),
                    operator: BinaryOperator::Add,
                    location: Location::new(7, 12),
                })),
                location: Location::new(0, 12)
            })
        );
    }
}

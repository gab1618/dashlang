use ast::{AssignmentExpr, BinaryExpr, Expr, Location, Symbol};
use pest::Parser;

use crate::{
    parser::{DashlangParser, Rule},
    utils::get_pair_location,
};

use super::{binary_operator::parse_binary_operator, parse_expression};

pub fn parse_compound_assign_expr(input: &str) -> AssignmentExpr {
    let ast = DashlangParser::parse(Rule::compound_assignment_expr, input)
        .expect("Could not parse compound assignment expression")
        .next()
        .expect("Could not get compound assignment expression");
    let (start, end) = get_pair_location(&ast);
    let mut ast_inner = ast.into_inner();

    let ast_symbol = ast_inner
        .next()
        .expect("Could not get symbol from compound assignment expression");
    let ast_operator = ast_inner
        .next()
        .expect("Could not get operator from compound assignment");
    let ast_operand = ast_inner
        .next()
        .expect("Could not get operand operator from compound assignment");
    let parsed_ast_operand = parse_expression(ast_operand.as_str());
    AssignmentExpr {
        symbol: ast_symbol.as_str().to_owned(),
        value: Box::new(Expr::BinaryExpr(Box::new(BinaryExpr {
            left: Expr::Symbol(Symbol {
                value: ast_symbol.as_str().to_owned(),
                location: Location::default(),
            }),
            right: parsed_ast_operand,
            operator: parse_binary_operator(ast_operator.as_str()),
            location: Location::default(),
        }))),
        location: Location::new(start, end),
    }
}

#[cfg(test)]
mod tests {
    use ast::{BinaryExpr, BinaryOperator, Int, Literal, Symbol};

    use super::*;
    #[test]
    fn test_compound_assignment() {
        assert_eq!(
            parse_compound_assign_expr("n += 1"),
            AssignmentExpr {
                symbol: String::from("n"),
                value: Box::new(Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Symbol(Symbol {
                        value: String::from("n"),
                        location: Location::default()
                    }),
                    right: Expr::Literal(Literal::Int(Int {
                        value: 1,
                        location: Location::new(0, 1)
                    })),
                    operator: BinaryOperator::Add,
                    location: Location::new(0, 0),
                }))),
                location: Location::new(0, 6),
            }
        );
        assert_eq!(
            parse_compound_assign_expr("x -= 5"),
            AssignmentExpr {
                symbol: String::from("x"),
                value: Box::new(Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Symbol(Symbol {
                        value: String::from("x"),
                        location: Location::default()
                    }),
                    right: Expr::Literal(Literal::Int(Int {
                        value: 5,
                        location: Location::new(0, 1)
                    })),
                    operator: BinaryOperator::Sub,
                    location: Location::default(),
                }))),
                location: Location::new(0, 6),
            }
        );
    }
}

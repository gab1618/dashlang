use ast::{AssignmentExpr, BinaryExpr, Expr, Location, Symbol};
use pest::Parser;

use crate::{
    errors::ParsingResult,
    parser::{DashlangParser, Rule},
    utils::get_pair_location,
};

use super::{binary_operator::parse_binary_operator, parse_expression};

pub fn parse_compound_assign_expr(
    input: &str,
    base_location: usize,
) -> ParsingResult<AssignmentExpr> {
    let ast = DashlangParser::parse(Rule::compound_assignment_expr, input)
        .expect("Could not parse compound assignment expression")
        .next()
        .expect("Could not get compound assignment expression");
    let (start, end) = get_pair_location(&ast);
    let mut ast_inner = ast.into_inner();

    let ast_symbol = ast_inner
        .next()
        .expect("Could not get symbol from compound assignment expression");
    let (symbol_start, symbol_end) = get_pair_location(&ast_symbol);
    let ast_operator = ast_inner
        .next()
        .expect("Could not get operator from compound assignment");
    let ast_operand = ast_inner
        .next()
        .expect("Could not get operand operator from compound assignment");
    let (start_ast_operand, _) = get_pair_location(&ast_operand);
    let parsed_ast_operand =
        parse_expression(ast_operand.as_str(), start_ast_operand + base_location)?;
    Ok(AssignmentExpr {
        symbol: ast_symbol.as_str().to_owned(),
        value: Box::new(Expr::BinaryExpr(Box::new(BinaryExpr {
            left: Expr::Symbol(Symbol {
                value: ast_symbol.as_str().to_owned(),
                location: Location::new(symbol_start + base_location, symbol_end + base_location),
            }),
            right: parsed_ast_operand,
            operator: parse_binary_operator(ast_operator.as_str()),
            location: Location::new(start + base_location, end + base_location),
        }))),
        location: Location::new(start + base_location, end + base_location),
    })
}

#[cfg(test)]
mod tests {
    use ast::{BinaryExpr, BinaryOperator, Int, Literal, Symbol};

    use super::*;
    #[test]
    fn test_compound_assignment() {
        assert_eq!(
            parse_compound_assign_expr("n += 1", 0),
            Ok(AssignmentExpr {
                symbol: String::from("n"),
                value: Box::new(Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Symbol(Symbol {
                        value: String::from("n"),
                        location: Location::new(0, 1)
                    }),
                    right: Expr::Literal(Literal::Int(Int {
                        value: 1,
                        location: Location::new(5, 6)
                    })),
                    operator: BinaryOperator::Add,
                    location: Location::new(0, 6),
                }))),
                location: Location::new(0, 6),
            })
        );
        assert_eq!(
            parse_compound_assign_expr("x -= 5", 0),
            Ok(AssignmentExpr {
                symbol: String::from("x"),
                value: Box::new(Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Symbol(Symbol {
                        value: String::from("x"),
                        location: Location::new(0, 1)
                    }),
                    right: Expr::Literal(Literal::Int(Int {
                        value: 5,
                        location: Location::new(5, 6)
                    })),
                    operator: BinaryOperator::Sub,
                    location: Location::new(0, 6),
                }))),
                location: Location::new(0, 6),
            })
        );
    }
}

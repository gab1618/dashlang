use ast::{Asignment, BinaryExpr, Expr};
use pest::Parser;

use crate::parser::{DashlangParser, Rule};

use super::{binary_operator::parse_binary_operator, parse_expression};

pub fn parse_compound_asign_expr(input: &str) -> Asignment {
    let ast = DashlangParser::parse(Rule::compound_asignment_expr, input)
        .expect("Could not parse compound asignment expression")
        .next()
        .expect("Could not get compound asignment expression");
    let mut ast_inner = ast.into_inner();

    let ast_symbol = ast_inner
        .next()
        .expect("Could not get symbol from compound asignment expression");
    let ast_operator = ast_inner
        .next()
        .expect("Could not get operator from compound asignment");
    let ast_operand = ast_inner
        .next()
        .expect("Could not get operand operator from compound asignment");
    let parsed_ast_operand = parse_expression(ast_operand.as_str());
    Asignment {
        symbol: ast_symbol.as_str().to_owned(),
        value: Box::new(Expr::BinaryExpr(Box::new(BinaryExpr {
            left: Expr::Symbol(ast_symbol.as_str().to_owned()),
            right: parsed_ast_operand,
            operator: parse_binary_operator(ast_operator.as_str()),
        }))),
    }
}

#[cfg(test)]
mod tests {
    use ast::{BinaryExpr, BinaryOperator, Literal};

    use super::*;
    #[test]
    fn test_compound_asignment() {
        assert_eq!(
            parse_compound_asign_expr("n += 1"),
            Asignment {
                symbol: String::from("n"),
                value: Box::new(Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Symbol(String::from("n")),
                    right: Expr::Literal(Literal::Int(1)),
                    operator: BinaryOperator::Add
                })))
            }
        );
        assert_eq!(
            parse_compound_asign_expr("x -= 5"),
            Asignment {
                symbol: String::from("x"),
                value: Box::new(Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Symbol(String::from("x")),
                    right: Expr::Literal(Literal::Int(5)),
                    operator: BinaryOperator::Sub
                })))
            }
        );
    }
}

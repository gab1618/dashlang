use pest::Parser;

use crate::{expression::parse_expression, DashlangParser, Rule};
use ast::Asignment;

pub fn parse_asignment_expression(input: &str) -> Asignment {
    let ast = DashlangParser::parse(Rule::asignment_expression, input)
        .expect("Could not parse asignment expression")
        .next()
        .expect("Could not parse asignment expression");
    let mut ast_inner = ast.into_inner();
    let ast_symbol = ast_inner
        .next()
        .expect("Could not get asignment expression symbol");
    let ast_value = ast_inner
        .next()
        .expect("Could not get asignment expression value");
    Asignment {
        symbol: ast_symbol.as_str().to_owned(),
        value: Box::new(parse_expression(ast_value.as_str())),
    }
}

#[cfg(test)]
mod tests {
    use ast::{BinaryExpr, BinaryOperator, Expr, Literal};

    use super::*;
    #[test]
    fn test_parse_value_asignment() {
        assert_eq!(
            parse_asignment_expression("age = 5"),
            Asignment {
                symbol: String::from("age"),
                value: Box::new(Expr::Literal(Literal::Int(5)))
            }
        );
    }
    #[test]
    fn test_parse_expr_asignment() {
        assert_eq!(
            parse_asignment_expression("age = 5 + 1"),
            Asignment {
                symbol: String::from("age"),
                value: Box::new(Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Literal(Literal::Int(5)),
                    right: Expr::Literal(Literal::Int(1)),
                    operator: BinaryOperator::Add
                })))
            }
        );
    }
}

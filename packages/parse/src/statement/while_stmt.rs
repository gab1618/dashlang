use ast::{Location, While};
use errors::DashlangResult;
use pest::Parser;

use crate::{
    body::parse_body,
    expression::parse_expression,
    parser::{DashlangParser, Rule},
    utils::get_pair_location,
};

pub fn parse_while_stmt(input: &str, base_location: usize) -> DashlangResult<While> {
    let ast = DashlangParser::parse(Rule::while_stmt, input)
        .expect("Could not parse while loop")
        .next()
        .unwrap();
    let (start, end) = get_pair_location(&ast);
    let mut inner_ast = ast.into_inner();
    let ast_cond = inner_ast
        .next()
        .expect("Could not get while statement condition");
    let (cond_start, _) = get_pair_location(&ast_cond);
    let parsed_cond = parse_expression(ast_cond.as_str(), cond_start + base_location)?;
    let ast_body = inner_ast
        .next()
        .expect("Could not get while statement body");
    let (body_start, _) = get_pair_location(&ast_body);
    let parsed_body = parse_body(ast_body.as_str(), body_start + base_location)?;
    Ok(While {
        cond: parsed_cond,
        body: parsed_body,
        location: Location::new(start + base_location, end + base_location),
    })
}
#[cfg(test)]
mod tests {
    use ast::{BinaryExpr, BinaryOperator, Boolean, Expr, Int, Literal, Symbol};

    use super::*;

    #[test]
    fn test_while_with_values() {
        assert_eq!(
            parse_while_stmt("while true {}", 0),
            Ok(While {
                cond: Expr::Literal(Literal::Bool(Boolean {
                    value: true,
                    location: Location::new(6, 10)
                })),
                body: vec![],
                location: Location::new(0, 13),
            })
        );
    }
    #[test]
    fn test_parse_while() {
        assert_eq!(
            parse_while_stmt("while count < 10 {}", 0),
            Ok(While {
                cond: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Symbol(Symbol {
                        value: String::from("count"),
                        location: Location::new(6, 11)
                    }),
                    right: Expr::Literal(Literal::Int(Int {
                        value: 10,
                        location: Location::new(14, 16)
                    })),
                    operator: BinaryOperator::Lt,
                    location: Location::new(6, 17),
                })),
                body: vec![],
                location: Location::new(0, 19),
            })
        );
    }
}

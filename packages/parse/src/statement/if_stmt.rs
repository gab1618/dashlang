use ast::{If, Location, Program, Stmt};
use errors::DashlangResult;
use pest::Parser;

use crate::{
    body::parse_body,
    expression::parse_expression,
    parser::{DashlangParser, Rule},
    utils::get_pair_location,
};

pub fn parse_if_stmt(input: &str, base_location: usize) -> DashlangResult<If> {
    let ast = DashlangParser::parse(Rule::if_stmt, input)
        .expect("Could not parse if statement")
        .next()
        .expect("Could not parse if statement");
    let (start, end) = get_pair_location(&ast);
    let mut inner_ast = ast.into_inner();
    let ast_cond = inner_ast.next().expect("Could not get condition");
    let (cond_start, _) = get_pair_location(&ast_cond);
    let parsed_cond = parse_expression(ast_cond.as_str(), cond_start + base_location)?;

    let ast_body = inner_ast.next().expect("Could not get scope");
    let (body_start, _) = get_pair_location(&ast_body);
    let parsed_body = parse_body(ast_body.as_str(), body_start + base_location)?;

    let ast_else = match inner_ast.next() {
        Some(pair) => {
            let (else_start, _) = get_pair_location(&pair);
            match pair.as_rule() {
                Rule::else_stmt => {
                    Some(parse_else_stmt(pair.as_str(), else_start + base_location)?)
                }
                Rule::else_if_stmt => Some(vec![Stmt::If(parse_else_if_stmt(
                    pair.as_str(),
                    else_start + base_location,
                )?)]),
                _ => unreachable!(),
            }
        }
        None => None,
    };
    Ok(If {
        cond: parsed_cond,
        body: parsed_body,
        else_block: ast_else,
        location: Location::new(start + base_location, end + base_location),
    })
}
fn parse_else_stmt(input: &str, base_location: usize) -> DashlangResult<Program> {
    let ast = DashlangParser::parse(Rule::else_stmt, input)
        .expect("Could not parse else statement")
        .next()
        .expect("Could not get else statement");
    let ast_body = ast.into_inner().next().expect("Could not get ast body");
    let (body_start, _) = get_pair_location(&ast_body);
    parse_body(ast_body.as_str(), body_start + base_location)
}
fn parse_else_if_stmt(input: &str, base_location: usize) -> DashlangResult<If> {
    let ast = DashlangParser::parse(Rule::else_if_stmt, input)
        .expect("Could not parse else if statement")
        .next()
        .expect("Could not get else if statement");
    let (start, end) = get_pair_location(&ast);
    let mut inner_ast = ast.into_inner();

    let ast_cond = inner_ast
        .next()
        .expect("Could not get else if statement condition");
    let (cond_start, _end) = get_pair_location(&ast_cond);
    let cond_expr = parse_expression(ast_cond.as_str(), cond_start + base_location)?;

    let ast_body = inner_ast
        .next()
        .expect("Could not get else if statement body");
    let (body_start, _end) = get_pair_location(&ast_body);
    let else_if_body = parse_body(ast_body.as_str(), body_start + base_location)?;

    let else_element: Option<Vec<Stmt>> = match inner_ast.next() {
        Some(element) => {
            let (element_start, _) = get_pair_location(&element);
            match element.as_rule() {
                Rule::else_stmt => Some(parse_else_stmt(
                    element.as_str(),
                    element_start + base_location,
                )?),
                Rule::else_if_stmt => Some(vec![Stmt::If(parse_else_if_stmt(
                    element.as_str(),
                    element_start + base_location,
                )?)]),
                _ => unreachable!(),
            }
        }
        None => None,
    };
    Ok(If {
        cond: cond_expr,
        body: else_if_body,
        else_block: else_element,
        location: Location::new(start + base_location, end + base_location),
    })
}
#[cfg(test)]
mod tests {

    use ast::{BinaryExpr, BinaryOperator, Boolean, Expr, Int, Literal, Return, Stmt, Symbol};

    use super::*;

    #[test]
    fn test_if_with_values() {
        assert_eq!(
            parse_if_stmt("if true {}", 0),
            Ok(If {
                cond: Expr::Literal(Literal::Bool(Boolean {
                    value: true,
                    location: Location::new(3, 7)
                })),
                body: vec![],
                else_block: None,
                location: Location::new(0, 10),
            })
        );
    }
    #[test]
    fn test_if_with_symbols() {
        assert_eq!(
            parse_if_stmt("if count < 10 {}", 0),
            Ok(If {
                cond: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Symbol(Symbol {
                        value: String::from("count"),
                        location: Location::new(3, 8)
                    }),
                    right: Expr::Literal(Literal::Int(Int {
                        value: 10,
                        location: Location::new(11, 13)
                    })),
                    operator: BinaryOperator::Lt,
                    location: Location::new(3, 14),
                })),
                body: vec![],
                else_block: None,
                location: Location::new(0, 16),
            })
        );
    }
    #[test]
    fn test_else() {
        assert_eq!(
            parse_if_stmt("if true {return true} else {return false}", 0),
            Ok(If {
                cond: Expr::Literal(Literal::Bool(Boolean {
                    value: true,
                    location: Location::new(3, 7)
                })),
                body: vec![Stmt::Return(Return {
                    value: Expr::Literal(Literal::Bool(Boolean {
                        value: true,
                        location: Location::new(16, 20)
                    })),
                    location: Location::new(9, 20)
                })],
                else_block: Some(vec![Stmt::Return(Return {
                    value: Expr::Literal(Literal::Bool(Boolean {
                        value: false,
                        location: Location::new(35, 40)
                    })),
                    location: Location::new(28, 40)
                })]),
                location: Location::new(0, 41),
            })
        );
    }
    #[test]
    fn test_if_else() {
        assert_eq!(
            parse_if_stmt(
                "if true {return true} else if true {return true} else {return false}",
                0
            ),
            Ok(If {
                cond: Expr::Literal(Literal::Bool(Boolean {
                    value: true,
                    location: Location::new(3, 7),
                })),
                body: vec![Stmt::Return(Return {
                    value: Expr::Literal(Literal::Bool(Boolean {
                        value: true,
                        location: Location::new(16, 20)
                    })),
                    location: Location::new(9, 20)
                })],
                else_block: Some(vec![Stmt::If(If {
                    cond: Expr::Literal(Literal::Bool(Boolean {
                        value: true,
                        location: Location::new(30, 34)
                    })),
                    body: vec![Stmt::Return(Return {
                        value: Expr::Literal(Literal::Bool(Boolean {
                            value: true,
                            location: Location::new(43, 47)
                        })),
                        location: Location::new(36, 47)
                    })],
                    else_block: Some(vec![Stmt::Return(Return {
                        value: Expr::Literal(Literal::Bool(Boolean {
                            value: false,
                            location: Location::new(62, 67)
                        })),
                        location: Location::new(55, 67)
                    })]),
                    location: Location::new(22, 68),
                })]),
                location: Location::new(0, 68),
            })
        );
    }
}

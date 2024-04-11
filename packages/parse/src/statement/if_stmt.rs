use ast::{If, Instruction, Location, Program, Stmt};
use pest::Parser;

use crate::{
    body::parse_body,
    expression::parse_expression,
    parser::{DashlangParser, Rule},
};

pub fn parse_if_stmt(input: &str) -> If {
    let ast = DashlangParser::parse(Rule::if_stmt, input)
        .expect("Could not parse if statement")
        .next()
        .expect("Could not parse if statement");
    let mut inner_ast = ast.into_inner();
    let ast_cond = parse_expression(inner_ast.next().expect("Could not get condition").as_str());
    let ast_body = parse_body(inner_ast.next().expect("Could not get scope").as_str());
    let ast_else = match inner_ast.next() {
        Some(pair) => match pair.as_rule() {
            Rule::else_stmt => Some(parse_else_stmt(pair.as_str())),
            Rule::else_if_stmt => Some(vec![Instruction::Stmt(Stmt::If(parse_else_if_stmt(
                pair.as_str(),
            )))]),
            _ => unreachable!(),
        },
        None => None,
    };
    If {
        cond: ast_cond,
        body: ast_body,
        else_block: ast_else,
        location: Location::default(),
    }
}
fn parse_else_stmt(input: &str) -> Program {
    let ast = DashlangParser::parse(Rule::else_stmt, input)
        .expect("Could not parse else statement")
        .next()
        .expect("Could not get else statement");
    parse_body(ast.into_inner().next().unwrap().as_str())
}
fn parse_else_if_stmt(input: &str) -> If {
    let ast = DashlangParser::parse(Rule::else_if_stmt, input)
        .expect("Could not parse else if statement")
        .next()
        .expect("Could not get else if statement");
    let mut inner_ast = ast.into_inner();
    let cond_expr = parse_expression(
        inner_ast
            .next()
            .expect("Could not get else if statement condition")
            .as_str(),
    );
    let else_if_body = parse_body(
        inner_ast
            .next()
            .expect("Could not get else if statement body")
            .as_str(),
    );
    let else_element: Option<Vec<Instruction>> = match inner_ast.next() {
        Some(element) => match element.as_rule() {
            Rule::else_stmt => Some(parse_else_stmt(element.as_str())),
            Rule::else_if_stmt => Some(vec![Instruction::Stmt(Stmt::If(parse_else_if_stmt(
                element.as_str(),
            )))]),
            _ => unreachable!(),
        },
        None => None,
    };
    If {
        cond: cond_expr,
        body: else_if_body,
        else_block: else_element,
        location: Location::default(),
    }
}
#[cfg(test)]
mod tests {

    use ast::{BinaryExpr, BinaryOperator, Boolean, Expr, Instruction, Int, Literal, Stmt, Symbol};

    use super::*;

    #[test]
    fn test_if_with_values() {
        assert_eq!(
            parse_if_stmt("if true {}"),
            If {
                cond: Expr::Literal(Literal::Bool(Boolean {
                    value: true,
                    location: Location::new(0, 4)
                })),
                body: vec![],
                else_block: None,
                location: Location::default(),
            }
        );
    }
    #[test]
    fn test_if_with_symbols() {
        assert_eq!(
            parse_if_stmt("if count < 10 {}"),
            If {
                cond: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Symbol(Symbol {
                        value: String::from("count"),
                        location: Location::default()
                    }),
                    right: Expr::Literal(Literal::Int(Int {
                        value: 10,
                        location: Location::new(0, 2)
                    })),
                    operator: BinaryOperator::Lt,
                    location: Location::default(),
                })),
                body: vec![],
                else_block: None,
                location: Location::default(),
            }
        );
        assert_eq!(
            parse_if_stmt("if count < 10 {return true}"),
            If {
                cond: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Symbol(Symbol {
                        value: String::from("count"),
                        location: Location::default()
                    }),
                    right: Expr::Literal(Literal::Int(Int {
                        value: 10,
                        location: Location::new(0, 2)
                    })),
                    operator: BinaryOperator::Lt,
                    location: Location::default(),
                })),
                body: vec![Instruction::Stmt(Stmt::Return(Expr::Literal(
                    Literal::Bool(Boolean {
                        value: true,
                        location: Location::new(0, 4)
                    })
                )))],
                else_block: None,
                location: Location::default(),
            }
        );
    }
    #[test]
    fn test_else() {
        assert_eq!(
            parse_if_stmt("if true {return true} else {return false}"),
            If {
                cond: Expr::Literal(Literal::Bool(Boolean {
                    value: true,
                    location: Location::new(0, 4)
                })),
                body: vec![Instruction::Stmt(Stmt::Return(Expr::Literal(
                    Literal::Bool(Boolean {
                        value: true,
                        location: Location::new(0, 4)
                    })
                )))],
                else_block: Some(vec![Instruction::Stmt(Stmt::Return(Expr::Literal(
                    Literal::Bool(Boolean {
                        value: false,
                        location: Location::new(0, 5)
                    })
                )))]),
                location: Location::default(),
            }
        );
    }
    #[test]
    fn test_if_else() {
        assert_eq!(
            parse_if_stmt("if true {return true} else if true {return true} else {return false}"),
            If {
                cond: Expr::Literal(Literal::Bool(Boolean {
                    value: true,
                    location: Location::new(0, 4),
                })),
                body: vec![Instruction::Stmt(Stmt::Return(Expr::Literal(
                    Literal::Bool(Boolean {
                        value: true,
                        location: Location::new(0, 4)
                    })
                )))],
                else_block: Some(vec![Instruction::Stmt(Stmt::If(If {
                    cond: Expr::Literal(Literal::Bool(Boolean {
                        value: true,
                        location: Location::new(0, 4)
                    })),
                    body: vec![Instruction::Stmt(Stmt::Return(Expr::Literal(
                        Literal::Bool(Boolean {
                            value: true,
                            location: Location::new(0, 4)
                        })
                    )))],
                    else_block: Some(vec![Instruction::Stmt(Stmt::Return(Expr::Literal(
                        Literal::Bool(Boolean {
                            value: false,
                            location: Location::new(0, 5)
                        })
                    )))]),
                    location: Location::default(),
                }))]),
                location: Location::default(),
            }
        );
    }
}

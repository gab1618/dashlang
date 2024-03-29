use ast::If;
use pest::Parser;

use crate::{
    expression::parse_expression,
    parser::{DashlangParser, Rule},
    scope::parse_scope,
};

pub fn parse_if_stmt(input: &str) -> If {
    let ast = DashlangParser::parse(Rule::if_stmt, input)
        .expect("Could not parse if statement")
        .next()
        .expect("Could not parse if statement");
    let mut inner_ast = ast.into_inner();
    let ast_cond = parse_expression(inner_ast.next().expect("Could not get condition").as_str());
    let ast_body = parse_scope(inner_ast.next().expect("Could not get scope").as_str());
    let ast_else = match inner_ast.next() {
        Some(pair) => Some(parse_scope(pair.into_inner().next().unwrap().as_str())),
        None => None,
    };
    If {
        cond: ast_cond,
        body: ast_body,
        else_block: ast_else,
    }
}
#[cfg(test)]
mod tests {

    use ast::{BinaryExpr, BinaryOperator, Expr, Instruction, Literal, Stmt};

    use super::*;

    #[test]
    fn test_if_with_values() {
        assert_eq!(
            parse_if_stmt("if true {}"),
            If {
                cond: Expr::Literal(Literal::Bool(true)),
                body: vec![],
                else_block: None
            }
        );
    }
    #[test]
    fn test_if_with_symbols() {
        assert_eq!(
            parse_if_stmt("if count < 10 {}"),
            If {
                cond: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Symbol(String::from("count")),
                    right: Expr::Literal(Literal::Int(10)),
                    operator: BinaryOperator::Lt
                })),
                body: vec![],
                else_block: None
            }
        );
        assert_eq!(
            parse_if_stmt("if count < 10 {return true}"),
            If {
                cond: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Symbol(String::from("count")),
                    right: Expr::Literal(Literal::Int(10)),
                    operator: BinaryOperator::Lt
                })),
                body: vec![Instruction::Stmt(Stmt::Return(Expr::Literal(
                    Literal::Bool(true)
                )))],
                else_block: None
            }
        );
    }
    #[test]
    fn test_else() {
        assert_eq!(
            parse_if_stmt("if true {return true} else {return false}"),
            If {
                cond: Expr::Literal(Literal::Bool(true)),
                body: vec![Instruction::Stmt(Stmt::Return(Expr::Literal(
                    Literal::Bool(true)
                )))],
                else_block: Some(vec![Instruction::Stmt(Stmt::Return(Expr::Literal(
                    Literal::Bool(false)
                )))])
            }
        );
    }
}

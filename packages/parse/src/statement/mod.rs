mod for_stmt;
mod if_stmt;
mod return_stmt;
mod while_stmt;
use ast::Stmt;
use pest::Parser;

use crate::parser::{DashlangParser, Rule};

use return_stmt::parse_return_stmt;

use self::{for_stmt::parse_for_stmt, if_stmt::parse_if_stmt, while_stmt::parse_while_stmt};

pub fn parse_statement(input: &str) -> Stmt {
    let ast = DashlangParser::parse(Rule::statement, input)
        .expect("Could not parse statement")
        .next()
        .expect("Could not parse statement");
    let ast_statement = ast.into_inner().next().expect("Could not get statement");
    match ast_statement.as_rule() {
        Rule::return_stmt => parse_return_stmt(ast_statement.as_str()),
        Rule::if_stmt => Stmt::If(parse_if_stmt(ast_statement.as_str())),
        Rule::while_stmt => Stmt::While(parse_while_stmt(ast_statement.as_str())),
        Rule::for_stmt => Stmt::For(Box::new(parse_for_stmt(ast_statement.as_str()))),
        _ => unreachable!(),
    }
}
#[cfg(test)]
mod tests {
    use ast::{BinaryExpr, BinaryOperator, Expr, If, Literal, Location, Stmt, While};

    use super::*;
    #[test]
    fn test_parse_ret_stmt() {
        assert_eq!(
            parse_statement("return 5"),
            Stmt::Return(Expr::Literal(Literal::Int(5)))
        );
    }
    #[test]
    fn test_parse_if() {
        assert_eq!(
            parse_statement("if count < 5 {}"),
            Stmt::If(If {
                cond: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Symbol(String::from("count")),
                    right: Expr::Literal(Literal::Int(5)),
                    operator: BinaryOperator::Lt,
                    location: Location::default(),
                })),
                else_block: None,
                body: vec![],
                location: Location::default(),
            })
        );
    }
    #[test]
    fn test_parse_while() {
        assert_eq!(
            parse_statement("while count < 5 {}"),
            Stmt::While(While {
                cond: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Symbol(String::from("count")),
                    right: Expr::Literal(Literal::Int(5)),
                    operator: BinaryOperator::Lt,
                    location: Location::default(),
                })),
                body: vec![],
                location: Location::default(),
            })
        );
    }
}

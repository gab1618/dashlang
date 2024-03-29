mod if_stmt;
mod print_stmt;
mod return_stmt;
mod while_stmt;
use ast::Stmt;
use pest::Parser;

use crate::parser::{DashlangParser, Rule};

use return_stmt::parse_return_stmt;

use self::{if_stmt::parse_if_stmt, print_stmt::parse_print_stmt, while_stmt::parse_while_stmt};

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
        Rule::print_stmt => parse_print_stmt(ast_statement.as_str()),
        _ => unreachable!(),
    }
}
#[cfg(test)]
mod tests {
    use ast::{BinaryExpr, BinaryOperator, Expr, If, Literal, Stmt, While};

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
                    operator: BinaryOperator::Lt
                })),
                else_block: None,
                body: vec![]
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
                    operator: BinaryOperator::Lt
                })),
                body: vec![]
            })
        );
    }
    #[test]
    fn test_parse_print() {
        assert_eq!(
            parse_statement("print name"),
            Stmt::Print(Expr::Symbol(String::from("name")))
        );
    }
}

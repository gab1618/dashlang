mod if_stmt;
mod print_stmt;
mod return_stmt;
mod while_stmt;
use ast::Stmt;
use pest::Parser;

use crate::parser::{DashlangParser, Rule};

use return_stmt::parse_return_stmt;

use self::{if_stmt::parse_if_stmt, while_stmt::parse_while_stmt};

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
        _ => unreachable!(),
    }
}
#[cfg(test)]
mod tests {
    use ast::{BinaryOp, BinaryOpType, Expr, If, Stmt, Value, While};

    use super::*;
    #[test]
    fn test_parse_ret_stmt() {
        assert_eq!(
            parse_statement("return 5"),
            Stmt::Return(Expr::Value(Value::Int(5)))
        );
    }
    #[test]
    fn test_parse_if() {
        assert_eq!(
            parse_statement("if count < 5 {}"),
            Stmt::If(If {
                cond: Expr::BinaryOp(Box::new(BinaryOp {
                    left: Expr::Symbol(String::from("count")),
                    right: Expr::Value(Value::Int(5)),
                    op_type: BinaryOpType::Lt
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
                cond: Expr::BinaryOp(Box::new(BinaryOp {
                    left: Expr::Symbol(String::from("count")),
                    right: Expr::Value(Value::Int(5)),
                    op_type: BinaryOpType::Lt
                })),
                body: vec![]
            })
        );
    }
}

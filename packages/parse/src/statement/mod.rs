mod if_stmt;
mod return_stmt;
mod while_stmt;
use ast::Stmt;
use pest::Parser;

use crate::parser::{DashlangParser, Rule};

use return_stmt::parse_return_stmt;

pub fn parse_statement(input: &str) -> Stmt {
    let ast = DashlangParser::parse(Rule::statement, input)
        .expect("Could not parse statement")
        .next()
        .expect("Could not parse statement");
    let ast_statement = ast.into_inner().next().expect("Could not get statement");
    match ast_statement.as_rule() {
        Rule::return_stmt => parse_return_stmt(ast_statement.as_str()),
        _ => unreachable!(),
    }
}
#[cfg(test)]
mod tests {
    use ast::{Expr, Stmt, Value};

    use super::*;
    #[test]
    fn test_parse_ret_stmt() {
        assert_eq!(
            parse_statement("return 5"),
            Stmt::Return(Expr::Value(Value::Int(5)))
        );
    }
}

use ast::{Expr, Stmt};
use pest::Parser;

use crate::{
    parser::{DashlangParser, Rule},
    values::parse_values,
};

pub fn parse_statement(input: &str) -> Stmt {
    let ast = DashlangParser::parse(Rule::statement, input)
        .expect("Could not parse statement")
        .next()
        .expect("Could not parse statement");
    let ast_statement = ast.into_inner().next().expect("Could not get statement");
    match ast_statement.as_rule() {
        Rule::return_stmt => {
            let return_stmt = ast_statement
                .into_inner()
                .next()
                .expect("Could not get return statement");
            let return_value = match return_stmt.as_rule() {
                Rule::value => {
                    let value = return_stmt
                        .into_inner()
                        .next()
                        .expect("Could not get value");
                    Expr::Value(parse_values(value.as_str()))
                }
                Rule::expression => todo!(),
                _ => unreachable!(),
            };
            Stmt::Return(return_value)
        }
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

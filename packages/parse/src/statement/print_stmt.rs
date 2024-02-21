use ast::{Expr, Stmt};
use pest::Parser;

use crate::{
    expression::parse_expression,
    parser::{DashlangParser, Rule},
    value::parse_values,
};

pub fn parse_print_stmt(input: &str) -> Stmt {
    let ast = DashlangParser::parse(Rule::print_stmt, input)
        .expect("Could not parse print statement")
        .next()
        .expect("Could not get print statement");
    let arg = ast
        .into_inner()
        .next()
        .expect("Could not get print statement arg");
    match arg.as_rule() {
        Rule::expression => Stmt::Print(parse_expression(arg.as_str())),
        Rule::value => Stmt::Print(Expr::Value(parse_values(arg.as_str()))),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use ast::{BinaryOp, BinaryOpType, Value};

    use super::*;
    #[test]
    fn test_parse_print() {
        assert_eq!(
            parse_print_stmt("print 18"),
            Stmt::Print(Expr::Value(Value::Int(18)))
        );
        assert_eq!(
            parse_print_stmt("print name"),
            Stmt::Print(Expr::Symbol(String::from("name")))
        );
        assert_eq!(
            parse_print_stmt("print age > 18"),
            Stmt::Print(Expr::BinaryOp(Box::new(BinaryOp {
                left: Expr::Symbol(String::from("age")),
                right: Expr::Value(Value::Int(18)),
                op_type: BinaryOpType::Gt
            })))
        );
    }
}

use pest::Parser;

use crate::{expression::parse_expression, value::parse_values, DashlangParser, Rule};
use ast::{Asignment, Expr, Value};

pub fn parse_asignment_expression(input: &str) -> Asignment {
    let mut final_asignment = Asignment {
        symbol: String::from(""),
        value: Box::new(Expr::Value(Value::Void)),
    };
    let ast = DashlangParser::parse(Rule::asignment_expression, input)
        .expect("Could not parse asignment expression")
        .next()
        .expect("Could not parse asignment expression");
    for item in ast.into_inner() {
        match item.as_rule() {
            Rule::value => {
                final_asignment.value = Box::new(Expr::Value(parse_values(item.as_str())));
            }
            Rule::symbol_name => {
                final_asignment.symbol = item.as_str().to_owned();
            }
            Rule::expression => {
                final_asignment.value = Box::new(parse_expression(item.as_str()));
            }
            _ => unreachable!(),
        }
    }
    final_asignment
}

#[cfg(test)]
mod tests {
    use ast::{BinaryOp, BinaryOpType};

    use super::*;
    #[test]
    fn test_parse_value_asignment() {
        assert_eq!(
            parse_asignment_expression("age = 5"),
            Asignment {
                symbol: String::from("age"),
                value: Box::new(Expr::Value(Value::Int(5)))
            }
        );
    }
    #[test]
    fn test_parse_expr_asignment() {
        assert_eq!(
            parse_asignment_expression("age = 5 + 1"),
            Asignment {
                symbol: String::from("age"),
                value: Box::new(Expr::BinaryOp(Box::new(BinaryOp {
                    left: Expr::Value(Value::Int(5)),
                    right: Expr::Value(Value::Int(1)),
                    op_type: BinaryOpType::Add
                })))
            }
        );
    }
}

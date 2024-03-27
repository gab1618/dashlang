use ast::{Call, Expr};
use pest::Parser;

use crate::{
    parser::{DashlangParser, Rule},
    value::parse_values,
};

use super::parse_expression;

pub fn parse_call_expression(input: &str) -> Call {
    let mut final_call = Call {
        symbol: String::new(),
        args: Vec::new(),
    };
    let ast = DashlangParser::parse(Rule::call_expression, input)
        .expect("Could not parse call expression")
        .next()
        .expect("Could not parse call expression");
    for element in ast.into_inner() {
        match element.as_rule() {
            Rule::symbol => final_call.symbol = element.as_str().to_owned(),
            Rule::call_arg => {
                let inner_arg = element
                    .into_inner()
                    .next()
                    .expect("Could not get call arg content");
                match inner_arg.as_rule() {
                    Rule::expression => {
                        final_call.args.push(parse_expression(inner_arg.as_str()));
                    }
                    Rule::value => {
                        final_call
                            .args
                            .push(Expr::Literal(parse_values(inner_arg.as_str())));
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
    final_call
}

#[cfg(test)]
mod tests {
    use ast::{Expr, Literal};

    use super::*;

    #[test]
    fn test_parse_call() {
        assert_eq!(
            parse_call_expression("println()"),
            Call {
                symbol: String::from("println"),
                args: vec![]
            }
        );
    }
    #[test]
    fn test_parse_call_with_args() {
        assert_eq!(
            parse_call_expression("println(18)"),
            Call {
                symbol: String::from("println"),
                args: vec![Expr::Literal(Literal::Int(18))]
            }
        );
        assert_eq!(
            parse_call_expression("println(name)"),
            Call {
                symbol: String::from("println"),
                args: vec![Expr::Symbol(String::from("name"))]
            }
        );
        assert_eq!(
            parse_call_expression("println(getName())"),
            Call {
                symbol: String::from("println"),
                args: vec![Expr::Call(Call {
                    symbol: String::from("getName"),
                    args: vec![]
                })]
            }
        );
        assert_eq!(
            parse_call_expression("println(getName(id))"),
            Call {
                symbol: String::from("println"),
                args: vec![Expr::Call(Call {
                    symbol: String::from("getName"),
                    args: vec![Expr::Symbol(String::from("id"))]
                })]
            }
        );
    }
}

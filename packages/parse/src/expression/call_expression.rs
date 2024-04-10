use ast::{Call, Expr, Location};
use pest::Parser;

use crate::parser::{DashlangParser, Rule};

use super::parse_expression;

pub fn parse_call_expression(input: &str) -> Call {
    let ast = DashlangParser::parse(Rule::call_expression, input)
        .expect("Could not parse call expression")
        .next()
        .expect("Could not parse call expression");
    let mut ast_inner = ast.into_inner();
    let symbol = ast_inner
        .next()
        .expect("Could not get call symbol")
        .as_str()
        .to_owned();
    let args: Vec<Expr> = ast_inner
        .map(|element| {
            let inner_arg = element
                .into_inner()
                .next()
                .expect("Could not get call arg content");
            parse_expression(inner_arg.as_str())
        })
        .collect();
    Call {
        symbol,
        args,
        location: Location::new(0, 0),
    }
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
                args: vec![],
                location: Location::new(0, 0)
            }
        );
    }
    #[test]
    fn test_parse_call_with_args() {
        assert_eq!(
            parse_call_expression("println(18)"),
            Call {
                symbol: String::from("println"),
                args: vec![Expr::Literal(Literal::Int(18))],
                location: Location::default()
            }
        );
        assert_eq!(
            parse_call_expression("println(name)"),
            Call {
                symbol: String::from("println"),
                args: vec![Expr::Symbol(String::from("name"))],
                location: Location::default()
            }
        );
        assert_eq!(
            parse_call_expression("println(getName())"),
            Call {
                symbol: String::from("println"),
                args: vec![Expr::Call(Call {
                    symbol: String::from("getName"),
                    args: vec![],
                    location: Location::default()
                })],
                location: Location::default()
            }
        );
        assert_eq!(
            parse_call_expression("println(getName(id))"),
            Call {
                symbol: String::from("println"),
                args: vec![Expr::Call(Call {
                    symbol: String::from("getName"),
                    args: vec![Expr::Symbol(String::from("id"))],
                    location: Location::default()
                })],
                location: Location::default()
            }
        );
    }
}

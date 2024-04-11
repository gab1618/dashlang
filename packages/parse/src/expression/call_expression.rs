use ast::{Call, Expr, Location};
use pest::Parser;

use crate::{
    parser::{DashlangParser, Rule},
    utils::get_pair_location,
};

use super::parse_expression;

pub fn parse_call_expression(input: &str) -> Call {
    let ast = DashlangParser::parse(Rule::call_expression, input)
        .expect("Could not parse call expression")
        .next()
        .expect("Could not parse call expression");
    let (start, end) = get_pair_location(&ast);
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
        location: Location::new(start, end),
    }
}

#[cfg(test)]
mod tests {
    use ast::{Expr, Int, Literal, Symbol};

    use super::*;

    #[test]
    fn test_parse_call() {
        assert_eq!(
            parse_call_expression("println()"),
            Call {
                symbol: String::from("println"),
                args: vec![],
                location: Location::new(0, 9)
            }
        );
    }
    #[test]
    fn test_parse_call_with_args() {
        assert_eq!(
            parse_call_expression("println(18)"),
            Call {
                symbol: String::from("println"),
                args: vec![Expr::Literal(Literal::Int(Int {
                    value: 18,
                    location: Location::new(0, 2)
                }))],
                location: Location::new(0, 11)
            }
        );
        assert_eq!(
            parse_call_expression("println(name)"),
            Call {
                symbol: String::from("println"),
                args: vec![Expr::Symbol(Symbol {
                    value: String::from("name"),
                    location: Location::default()
                })],
                location: Location::new(0, 13)
            }
        );
        assert_eq!(
            parse_call_expression("println(getName())"),
            Call {
                symbol: String::from("println"),
                args: vec![Expr::Call(Call {
                    symbol: String::from("getName"),
                    args: vec![],
                    location: Location::new(0, 9)
                })],
                location: Location::new(0, 18)
            }
        );
        assert_eq!(
            parse_call_expression("println(getName(id))"),
            Call {
                symbol: String::from("println"),
                args: vec![Expr::Call(Call {
                    symbol: String::from("getName"),
                    args: vec![Expr::Symbol(Symbol {
                        value: String::from("id"),
                        location: Location::default()
                    })],
                    location: Location::new(0, 11)
                })],
                location: Location::new(0, 20)
            }
        );
    }
}

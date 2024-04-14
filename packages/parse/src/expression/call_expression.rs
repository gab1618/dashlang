use ast::{Call, Expr, Location};
use pest::Parser;

use crate::{
    parser::{DashlangParser, Rule},
    utils::get_pair_location,
};

use super::parse_expression;

pub fn parse_call_expression(input: &str, base_location: usize) -> Call {
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
            let (arg_start, _) = get_pair_location(&inner_arg);
            parse_expression(inner_arg.as_str(), arg_start + base_location)
        })
        .collect();
    Call {
        symbol,
        args,
        location: Location::new(start + base_location, end + base_location),
    }
}

#[cfg(test)]
mod tests {
    use ast::{Expr, Int, Literal, Symbol};

    use super::*;

    #[test]
    fn test_parse_call() {
        assert_eq!(
            parse_call_expression("println()", 0),
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
            parse_call_expression("println(18)", 0),
            Call {
                symbol: String::from("println"),
                args: vec![Expr::Literal(Literal::Int(Int {
                    value: 18,
                    location: Location::new(8, 10)
                }))],
                location: Location::new(0, 11)
            }
        );
        assert_eq!(
            parse_call_expression("println(name)", 0),
            Call {
                symbol: String::from("println"),
                args: vec![Expr::Symbol(Symbol {
                    value: String::from("name"),
                    location: Location::new(8, 12)
                })],
                location: Location::new(0, 13)
            }
        );
        assert_eq!(
            parse_call_expression("println(getName())", 0),
            Call {
                symbol: String::from("println"),
                args: vec![Expr::Call(Call {
                    symbol: String::from("getName"),
                    args: vec![],
                    location: Location::new(8, 17)
                })],
                location: Location::new(0, 18)
            }
        );
        assert_eq!(
            parse_call_expression("println(getName(id))", 0),
            Call {
                symbol: String::from("println"),
                args: vec![Expr::Call(Call {
                    symbol: String::from("getName"),
                    args: vec![Expr::Symbol(Symbol {
                        value: String::from("id"),
                        location: Location::new(16, 18)
                    })],
                    location: Location::new(8, 19)
                })],
                location: Location::new(0, 20)
            }
        );
    }
}

use ast::{DestructuringAsignment, Symbol};
use errors::DashlangResult;
use pest::Parser;

use crate::{
    parser::{DashlangParser, Rule},
    utils::get_pair_location,
};

use super::parse_expression;

pub fn parse_destructuring_assignment(
    input: &str,
    base_location: usize,
) -> DashlangResult<DestructuringAsignment> {
    let ast = DashlangParser::parse(Rule::destructuring_assignment, input)
        .expect("Could not parse tuple destructuring")
        .next()
        .expect("Could not parse tuple destructuring");
    let (start, end) = get_pair_location(&ast);
    let mut ast_inner = ast.into_inner();
    let ast_symbols = ast_inner.next().expect("Could not get symbols");
    let ast_value = ast_inner.next().expect("Could not get value");
    let (ast_value_start, _) = get_pair_location(&ast_value);
    Ok(DestructuringAsignment {
        location: (start + base_location, end + base_location).into(),
        symbols: ast_symbols
            .into_inner()
            .map(|element| {
                let (element_start, element_end) = get_pair_location(&element);
                Symbol {
                    value: element.as_str().to_owned(),
                    location: (element_start + base_location, element_end + base_location).into(),
                }
            })
            .collect(),
        value: Box::new(parse_expression(
            ast_value.as_str(),
            ast_value_start + base_location,
        )?),
    })
}

#[cfg(test)]
mod tests {
    use ast::{Expr, Literal, Symbol, Tuple};

    use super::*;

    #[test]
    fn test_parse_destructuring_assignment() {
        assert_eq!(
            parse_destructuring_assignment("(first, second) = (name, age)", 0),
            Ok(DestructuringAsignment {
                location: (0, 29).into(),
                symbols: vec![
                    Symbol {
                        value: "first".to_owned(),
                        location: (1, 6).into()
                    },
                    Symbol {
                        value: "second".to_owned(),
                        location: (8, 14).into()
                    }
                ],
                value: Box::new(Expr::Literal(Literal::Tuple(Tuple {
                    value: vec![
                        Expr::Symbol(Symbol {
                            value: "name".to_owned(),
                            location: (19, 23).into()
                        }),
                        Expr::Symbol(Symbol {
                            value: "age".to_owned(),
                            location: (25, 28).into()
                        })
                    ],
                    location: (18, 29).into()
                })))
            })
        );
    }
}

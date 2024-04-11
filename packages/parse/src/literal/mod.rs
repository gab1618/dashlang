use ast::{Boolean, Closure, Float, Int, Literal, Location, Str, Vector};
use pest::Parser;

use crate::body::parse_body;
use crate::expression::parse_expression;
use crate::parser::{DashlangParser, Rule};
use crate::utils::get_pair_location;

pub fn parse_literal(input: &str) -> Literal {
    let parsed = DashlangParser::parse(Rule::literal, input)
        .expect("Could not parse value")
        .next()
        .expect("Could not parse value");
    let (start, end) = get_pair_location(&parsed);
    if parsed.as_rule() != Rule::literal {
        panic!("Expected rule to be value");
    }
    let inner_value = parsed.into_inner().next().expect("Could not parse value");
    match inner_value.as_rule() {
        Rule::int => {
            let parsed: i64 = inner_value
                .as_str()
                .parse()
                .expect("Could not parse integer value");
            Literal::Int(Int {
                value: parsed,
                location: Location::new(start, end),
            })
        }
        Rule::float => {
            let parsed: f64 = inner_value
                .as_str()
                .parse()
                .expect("Could not parse float value");
            Literal::Float(Float {
                value: parsed,
                location: Location::new(start, end),
            })
        }
        Rule::boolean => {
            let val = inner_value.as_str() == "true";
            Literal::Bool(Boolean {
                value: val,
                location: Location::new(start, end),
            })
        }
        Rule::string => Literal::String(Str {
            value: inner_value
                .into_inner()
                .next()
                .expect("Could not parse string")
                .as_str()
                .to_owned(),
            location: Location::new(start, end),
        }),
        Rule::closure => {
            let mut inner_ast = inner_value.into_inner();
            let params: Vec<String> = inner_ast
                .next()
                .expect("Could not get closure params")
                .into_inner()
                .map(|component| component.as_str().to_owned())
                .collect();
            let body = parse_body(
                inner_ast
                    .next()
                    .expect("Could not get closure body")
                    .as_str(),
            );
            Literal::Closure(Closure {
                params,
                body,
                location: Location::new(start, end),
            })
        }
        Rule::vector => {
            let inner_ast = inner_value.into_inner();
            Literal::Vector(Vector {
                value: inner_ast
                    .map(|element| parse_expression(element.as_str()))
                    .collect(),
                location: Location::new(start, end),
            })
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use ast::{Closure, Expr, Instruction, Stmt};

    use super::*;
    #[test]
    fn parse_value() {
        assert_eq!(
            parse_literal("10"),
            Literal::Int(Int {
                value: 10,
                location: Location::new(0, 2)
            })
        );
        assert_eq!(
            parse_literal("-10"),
            Literal::Int(Int {
                value: -10,
                location: Location::new(0, 3)
            })
        );
        assert_eq!(
            parse_literal("10.5"),
            Literal::Float(Float {
                value: 10.5,
                location: Location::new(0, 4)
            })
        );
        assert_eq!(
            parse_literal("-10.5"),
            Literal::Float(Float {
                value: -10.5,
                location: Location::new(0, 5)
            })
        );
        assert_eq!(
            parse_literal("true"),
            Literal::Bool(Boolean {
                value: true,
                location: Location::new(0, 4)
            })
        );
        assert_eq!(
            parse_literal("false"),
            Literal::Bool(Boolean {
                value: false,
                location: Location::new(0, 5)
            })
        );
        assert_eq!(
            parse_literal(r#""apple""#),
            Literal::String(Str {
                value: String::from("apple"),
                location: Location::new(0, 7)
            })
        );
        assert_eq!(
            parse_literal(r#""green apple""#),
            Literal::String(Str {
                value: "green apple".to_owned(),
                location: Location::new(0, 13)
            })
        );
        assert_eq!(
            parse_literal("(name, age) {return true}"),
            Literal::Closure(Closure {
                params: vec![String::from("name"), String::from("age")],
                body: vec![Instruction::Stmt(Stmt::Return(Expr::Literal(
                    Literal::Bool(Boolean {
                        value: true,
                        location: Location::new(0, 4)
                    })
                )))],
                location: Location::new(0, 25)
            })
        );
    }
    #[test]
    fn test_parse_vector() {
        assert_eq!(
            parse_literal("[1, 8, 7]"),
            Literal::Vector(Vector {
                value: vec![
                    Expr::Literal(Literal::Int(Int {
                        value: 1,
                        location: Location::new(0, 1)
                    })),
                    Expr::Literal(Literal::Int(Int {
                        value: 8,
                        location: Location::new(0, 1)
                    })),
                    Expr::Literal(Literal::Int(Int {
                        value: 7,
                        location: Location::new(0, 1)
                    })),
                ],
                location: Location::new(0, 9)
            })
        );
    }
}

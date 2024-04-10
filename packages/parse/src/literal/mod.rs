use ast::{Closure, Literal, Location};
use pest::Parser;

use crate::body::parse_body;
use crate::expression::parse_expression;
use crate::parser::{DashlangParser, Rule};

pub fn parse_literal(input: &str) -> Literal {
    let parsed = DashlangParser::parse(Rule::literal, input)
        .expect("Could not parse value")
        .next()
        .expect("Could not parse value");
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
            Literal::Int(parsed)
        }
        Rule::float => {
            let parsed: f64 = inner_value
                .as_str()
                .parse()
                .expect("Could not parse float value");
            Literal::Float(parsed)
        }
        Rule::boolean => {
            let val = inner_value.as_str() == "true";
            Literal::Bool(val)
        }
        Rule::string => Literal::String(
            inner_value
                .into_inner()
                .next()
                .expect("Could not parse string")
                .as_str()
                .to_owned(),
        ),
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
                location: Location::default(),
            })
        }
        Rule::vector => {
            let inner_ast = inner_value.into_inner();
            Literal::Vector(
                inner_ast
                    .map(|element| parse_expression(element.as_str()))
                    .collect(),
            )
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
        assert_eq!(parse_literal("10"), Literal::Int(10));
        assert_eq!(parse_literal("-10"), Literal::Int(-10));
        assert_eq!(parse_literal("10.5"), Literal::Float(10.5));
        assert_eq!(parse_literal("-10.5"), Literal::Float(-10.5));
        assert_eq!(parse_literal("true"), Literal::Bool(true));
        assert_eq!(parse_literal("false"), Literal::Bool(false));
        assert_eq!(
            parse_literal(r#""apple""#),
            Literal::String(String::from("apple"))
        );
        assert_eq!(
            parse_literal(r#""green apple""#),
            Literal::String(String::from("green apple"))
        );
        assert_eq!(
            parse_literal("(name, age) {return true}"),
            Literal::Closure(Closure {
                params: vec![String::from("name"), String::from("age")],
                body: vec![Instruction::Stmt(Stmt::Return(Expr::Literal(
                    Literal::Bool(true)
                )))],
                location: Location::default(),
            })
        );
    }
    #[test]
    fn test_parse_vector() {
        assert_eq!(
            parse_literal("[1, 8, 7]"),
            Literal::Vector(vec![
                Expr::Literal(Literal::Int(1)),
                Expr::Literal(Literal::Int(8)),
                Expr::Literal(Literal::Int(7)),
            ])
        );
    }
}

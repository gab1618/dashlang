use ast::{Expr, Instruction, Program, Value};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "dashlang.pest"]
struct DashlangParser {}

pub fn parse(input: &str) -> Program {
    todo!()
}
pub fn parse_values(input: &str) -> Value {
    let parsed = DashlangParser::parse(Rule::value, input)
        .expect("Could not parse value")
        .next()
        .expect("Could not parse value");
    if parsed.as_rule() != Rule::value {
        panic!("Expected rule to be value");
    }
    let inner_value = parsed.into_inner().next().expect("Could not parse value");
    match inner_value.as_rule() {
        Rule::int => {
            let parsed: i64 = inner_value
                .as_str()
                .parse()
                .expect("Could not parse integer value");
            Value::Int(parsed)
        }
        Rule::float => {
            let parsed: f64 = inner_value
                .as_str()
                .parse()
                .expect("Could not parse float value");
            Value::Float(parsed)
        }
        Rule::boolean => {
            let val = inner_value.as_str() == "true";
            Value::Bool(val)
        }
        Rule::string => Value::String(
            inner_value
                .into_inner()
                .next()
                .expect("Could not parse string")
                .as_str()
                .to_owned(),
        ),
        Rule::closure => todo!(),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_value() {
        assert_eq!(parse_values("10"), Value::Int(10));
        assert_eq!(parse_values("10.5"), Value::Float(10.5));
        assert_eq!(parse_values("true"), Value::Bool(true));
        assert_eq!(parse_values("false"), Value::Bool(false));
        assert_eq!(
            parse_values(r#""apple""#),
            Value::String(String::from("apple"))
        );
        assert_eq!(
            parse_values(r#""green apple""#),
            Value::String(String::from("green apple"))
        );
    }
}

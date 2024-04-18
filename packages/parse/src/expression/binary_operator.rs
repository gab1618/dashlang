use ast::BinaryOperator;
use errors::{DashlangError, DashlangResult, ErrorKind, ParsingErrorKind};

pub fn parse_binary_operator(input: &str) -> DashlangResult<BinaryOperator> {
    match input {
        "+" => Ok(BinaryOperator::Add),
        "-" => Ok(BinaryOperator::Sub),
        "*" => Ok(BinaryOperator::Mul),
        "/" => Ok(BinaryOperator::Div),
        ">" => Ok(BinaryOperator::Gt),
        ">=" => Ok(BinaryOperator::Ge),
        "<" => Ok(BinaryOperator::Lt),
        "<=" => Ok(BinaryOperator::Le),
        "==" => Ok(BinaryOperator::Eq),
        "&&" => Ok(BinaryOperator::And),
        "||" => Ok(BinaryOperator::Or),
        _ => Err(DashlangError {
            location: None,
            message: "Invalid operator".to_owned(),
            kind: ErrorKind::Parsing(ParsingErrorKind::Default),
        }),
    }
}

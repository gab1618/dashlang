use ast::BinaryOperator;

pub fn parse_binary_operator(input: &str) -> BinaryOperator {
    match input {
        "+" => BinaryOperator::Add,
        "-" => BinaryOperator::Sub,
        "*" => BinaryOperator::Mul,
        "/" => BinaryOperator::Div,
        ">" => BinaryOperator::Gt,
        ">=" => BinaryOperator::Ge,
        "<" => BinaryOperator::Lt,
        "<=" => BinaryOperator::Le,
        "==" => BinaryOperator::Eq,
        "&&" => BinaryOperator::And,
        "||" => BinaryOperator::Or,
        _ => panic!("Invalid operator"),
    }
}

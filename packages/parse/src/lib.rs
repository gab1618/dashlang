use ast::{BinaryOp, BinaryOpType, Closure, Expr, Instruction, Program, Stmt, Value};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "dashlang.pest"]
struct DashlangParser {}

pub fn parse(input: &str) -> Program {
    todo!()
}
fn parse_scope(input: &str) -> Program {
    let mut body: Program = vec![];
    let ast = DashlangParser::parse(Rule::scope, input)
        .expect("Could not parse scope")
        .next()
        .expect("Could not parse scope");
    for instruction in ast.into_inner() {
        let parsed_instruction = parse_instruction(instruction.as_str());
        body.push(parsed_instruction);
    }
    body
}
#[derive(Debug, Clone, PartialEq)]
enum BinaryExpressionToken {
    Value(Value),
    Expr(Expr),
    Operator(BinaryOpType),
}
fn parse_binary_expression(input: &str) -> BinaryOp {
    let ast = DashlangParser::parse(Rule::binary_expression, input)
        .expect("Could not parse binary expression")
        .next()
        .expect("Could not parse binary expression");
    let mut flat_expression: Vec<BinaryExpressionToken> = vec![];
    for element in ast.into_inner() {
        match element.as_rule() {
            Rule::binary_operator => match element.as_str() {
                "+" => flat_expression.push(BinaryExpressionToken::Operator(BinaryOpType::Add)),
                "-" => flat_expression.push(BinaryExpressionToken::Operator(BinaryOpType::Sub)),
                "*" => flat_expression.push(BinaryExpressionToken::Operator(BinaryOpType::Mul)),
                "/" => flat_expression.push(BinaryExpressionToken::Operator(BinaryOpType::Div)),
                _ => unreachable!(),
            },
            Rule::value => {
                flat_expression.push(BinaryExpressionToken::Value(parse_values(element.as_str())));
            }
            _ => unreachable!(),
        }
    }
    flat_binary_expression_to_ast(&mut flat_expression)
}
fn flat_binary_expression_to_ast(flat_expression: &mut Vec<BinaryExpressionToken>) -> BinaryOp {
    while flat_expression.len() > 1 {
        merge_flat_binary_op_tokens_by_operations(
            flat_expression,
            &vec![
                BinaryExpressionToken::Operator(BinaryOpType::Mul),
                BinaryExpressionToken::Operator(BinaryOpType::Div),
            ],
        );
        merge_flat_binary_op_tokens_by_operations(
            flat_expression,
            &vec![
                BinaryExpressionToken::Operator(BinaryOpType::Add),
                BinaryExpressionToken::Operator(BinaryOpType::Sub),
            ],
        )
    }
    match flat_expression
        .into_iter()
        .next()
        .expect("Expected expression to ended with at least 1 element")
    {
        BinaryExpressionToken::Expr(expr) => match expr {
            Expr::BinaryOp(op) => *op.to_owned(),
            _ => panic!("Expected expression to be binary operation"),
        },
        BinaryExpressionToken::Value(_) => {
            panic!("Expected binary operation to not have just a value")
        }
        BinaryExpressionToken::Operator(_) => {
            panic!("Expected binary operation to not have just a operator")
        }
    }
}
fn merge_flat_binary_op_tokens_by_operations(
    flat_expression: &mut Vec<BinaryExpressionToken>,
    allowed: &Vec<BinaryExpressionToken>,
) {
    for (pos, token) in flat_expression.clone().into_iter().enumerate() {
        if allowed.contains(&token) {
            if let BinaryExpressionToken::Operator(op) = token {
                merge_flat_binary_op_tokens(flat_expression, pos, op);
                merge_flat_binary_op_tokens_by_operations(flat_expression, allowed);
                break;
            }
        }
    }
}
fn merge_flat_binary_op_tokens(
    flat_expression: &mut Vec<BinaryExpressionToken>,
    operator_pos: usize,
    op: BinaryOpType,
) {
    let (previous_element, next_element): (BinaryExpressionToken, BinaryExpressionToken) = {
        let next = flat_expression.remove(operator_pos + 1);
        let previous = flat_expression.remove(operator_pos - 1);
        (previous, next)
    };
    let _ = std::mem::replace(
        &mut flat_expression[operator_pos - 1], // Since we removed the previous item, we use position - 1
        BinaryExpressionToken::Expr(Expr::BinaryOp(Box::new(BinaryOp {
            left: match previous_element {
                BinaryExpressionToken::Value(val) => Expr::Value(val.clone()),
                BinaryExpressionToken::Expr(expr) => expr,
                BinaryExpressionToken::Operator(_) => {
                    panic!("Expected token after operator to be a value or expression")
                }
            },
            right: match next_element {
                BinaryExpressionToken::Value(val) => Expr::Value(val.clone()),
                BinaryExpressionToken::Expr(expr) => expr,
                BinaryExpressionToken::Operator(_) => {
                    panic!("Expected token after operator to be a value or expression")
                }
            },
            op_type: op,
        }))),
    );
}
fn parse_instruction(input: &str) -> Instruction {
    let ast = DashlangParser::parse(Rule::instruction, input)
        .expect("Could not parse instruction")
        .next()
        .expect("Could not parse instruction");
    let instruction_type = ast
        .into_inner()
        .next()
        .expect("Could not get instruction type");
    match instruction_type.as_rule() {
        Rule::statement => {
            let inner_statement = instruction_type
                .into_inner()
                .next()
                .expect("Could not get statement value");
            match inner_statement.as_rule() {
                Rule::return_stmt => {
                    let return_stmt = inner_statement
                        .into_inner()
                        .next()
                        .expect("Could not get return statement");
                    let return_value = match return_stmt.as_rule() {
                        Rule::value => {
                            let value = return_stmt
                                .into_inner()
                                .next()
                                .expect("Could not get value");
                            Expr::Value(parse_values(value.as_str()))
                        }
                        Rule::expression => todo!(),
                        _ => unreachable!(),
                    };
                    Instruction::Stmt(Stmt::Return(return_value))
                }
                _ => unreachable!(),
            }
        }
        Rule::expression => {
            let inner_expression = instruction_type
                .into_inner()
                .next()
                .expect("Could not get expression value");
            match inner_expression.as_rule() {
                Rule::binary_expression => {
                    let parsed = parse_binary_expression(inner_expression.as_str());
                    Instruction::Expr(Expr::BinaryOp(Box::new(parsed)))
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
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
        Rule::closure => {
            let mut params: Vec<String> = vec![];
            let mut body: Program = vec![];
            for component in inner_value.into_inner() {
                match component.as_rule() {
                    Rule::closure_params => {
                        for param in component.into_inner() {
                            params.push(param.as_str().to_owned());
                        }
                    }
                    Rule::scope => {
                        let parsed = parse_scope(component.as_str());
                        for instruction in parsed {
                            body.push(instruction);
                        }
                    }
                    _ => unreachable!(),
                }
            }
            Value::Closure(Closure { params, body })
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use ast::{BinaryOpType, Closure, Stmt};

    use super::*;
    #[test]
    fn parse_value() {
        assert_eq!(parse_values("10"), Value::Int(10));
        assert_eq!(parse_values("-10"), Value::Int(-10));
        assert_eq!(parse_values("10.5"), Value::Float(10.5));
        assert_eq!(parse_values("-10.5"), Value::Float(-10.5));
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
        assert_eq!(
            parse_values("(name, age) {return true}"),
            Value::Closure(Closure {
                params: vec![String::from("name"), String::from("age")],
                body: vec![Instruction::Stmt(Stmt::Return(Expr::Value(Value::Bool(
                    true
                ))))]
            })
        );
    }
    #[test]
    fn test_parse_binary_op() {
        assert_eq!(
            parse_binary_expression("1 * 2"),
            BinaryOp {
                left: Expr::Value(Value::Int(1)),
                right: Expr::Value(Value::Int(2)),
                op_type: BinaryOpType::Mul
            }
        );
        assert_eq!(
            parse_binary_expression("1 + 2"),
            BinaryOp {
                left: Expr::Value(Value::Int(1)),
                right: Expr::Value(Value::Int(2)),
                op_type: BinaryOpType::Add
            }
        );
        assert_eq!(
            parse_binary_expression("1 + 2 * 2"),
            BinaryOp {
                left: Expr::Value(Value::Int(1)),
                right: Expr::BinaryOp(Box::new(BinaryOp {
                    left: Expr::Value(Value::Int(2)),
                    right: Expr::Value(Value::Int(2)),
                    op_type: BinaryOpType::Mul
                })),
                op_type: BinaryOpType::Add
            }
        );
        assert_eq!(
            parse_binary_expression("1 + 2 * 2 / 2"),
            BinaryOp {
                left: Expr::Value(Value::Int(1)),
                right: Expr::BinaryOp(Box::new(BinaryOp {
                    left: Expr::BinaryOp(Box::new(BinaryOp {
                        left: Expr::Value(Value::Int(2)),
                        right: Expr::Value(Value::Int(2)),
                        op_type: BinaryOpType::Mul
                    })),
                    right: Expr::Value(Value::Int(2)),
                    op_type: BinaryOpType::Div
                })),
                op_type: BinaryOpType::Add
            }
        );
    }
    #[test]
    fn test_parse_binary_expr() {
        assert_eq!(
            parse_instruction("1 + 2"),
            Instruction::Expr(Expr::BinaryOp(Box::new(BinaryOp {
                left: Expr::Value(Value::Int(1)),
                right: Expr::Value(Value::Int(2)),
                op_type: BinaryOpType::Add
            })))
        );
    }
}

use super::{binary_operator::parse_binary_operator, parse_expression};
use crate::{literal::parse_literal, DashlangParser, Rule};
use ast::{BinaryExpr, BinaryOperator, Expr, Literal};
use pest::Parser;

#[derive(Debug, Clone, PartialEq)]
enum BinaryExpressionToken {
    Literal(Literal),
    Expr(Expr),
    Operator(BinaryOperator),
}
pub fn parse_binary_expression(input: &str) -> BinaryExpr {
    let ast = DashlangParser::parse(Rule::binary_expression, input)
        .expect("Could not parse binary expression")
        .next()
        .expect("Could not parse binary expression");
    let mut flat_expression: Vec<BinaryExpressionToken> = vec![];
    for element in ast.into_inner() {
        match element.as_rule() {
            Rule::binary_operator => flat_expression.push(BinaryExpressionToken::Operator(
                parse_binary_operator(element.as_str()),
            )),
            Rule::literal => {
                flat_expression.push(BinaryExpressionToken::Literal(parse_literal(
                    element.as_str(),
                )));
            }
            Rule::expression => {
                let parsed = parse_expression(element.as_str());
                flat_expression.push(BinaryExpressionToken::Expr(parsed));
            }
            Rule::symbol => {
                let parsed = element.as_str().to_owned();
                flat_expression.push(BinaryExpressionToken::Expr(Expr::Symbol(parsed)));
            }
            Rule::call_expression => {
                let parsed = parse_expression(element.as_str());
                flat_expression.push(BinaryExpressionToken::Expr(parsed));
            }
            _ => unreachable!(),
        }
    }
    flat_binary_expression_to_ast(&mut flat_expression)
}
fn flat_binary_expression_to_ast(flat_expression: &mut Vec<BinaryExpressionToken>) -> BinaryExpr {
    while flat_expression.len() > 1 {
        merge_flat_binary_op_tokens_by_operations(
            flat_expression,
            &vec![
                BinaryExpressionToken::Operator(BinaryOperator::Mul),
                BinaryExpressionToken::Operator(BinaryOperator::Div),
            ],
        );
        merge_flat_binary_op_tokens_by_operations(
            flat_expression,
            &vec![
                BinaryExpressionToken::Operator(BinaryOperator::Add),
                BinaryExpressionToken::Operator(BinaryOperator::Sub),
            ],
        );
        merge_flat_binary_op_tokens_by_operations(
            flat_expression,
            &vec![
                BinaryExpressionToken::Operator(BinaryOperator::Gt),
                BinaryExpressionToken::Operator(BinaryOperator::Ge),
                BinaryExpressionToken::Operator(BinaryOperator::Lt),
                BinaryExpressionToken::Operator(BinaryOperator::Le),
                BinaryExpressionToken::Operator(BinaryOperator::Eq),
            ],
        );
        merge_flat_binary_op_tokens_by_operations(
            flat_expression,
            &vec![
                BinaryExpressionToken::Operator(BinaryOperator::Or),
                BinaryExpressionToken::Operator(BinaryOperator::And),
            ],
        );
    }
    match flat_expression
        .into_iter()
        .next()
        .expect("Expected expression to ended with at least 1 element")
    {
        BinaryExpressionToken::Expr(expr) => match expr {
            Expr::BinaryExpr(op) => *op.to_owned(),
            _ => panic!("Expected expression to be binary operation"),
        },
        BinaryExpressionToken::Literal(_) => {
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
    op: BinaryOperator,
) {
    let (previous_element, next_element): (BinaryExpressionToken, BinaryExpressionToken) = {
        let next = flat_expression.remove(operator_pos + 1);
        let previous = flat_expression.remove(operator_pos - 1);
        (previous, next)
    };
    let _ = std::mem::replace(
        &mut flat_expression[operator_pos - 1], // Since we removed the previous item, we use position - 1
        BinaryExpressionToken::Expr(Expr::BinaryExpr(Box::new(BinaryExpr {
            left: match previous_element {
                BinaryExpressionToken::Literal(val) => Expr::Literal(val.clone()),
                BinaryExpressionToken::Expr(expr) => expr,
                BinaryExpressionToken::Operator(_) => {
                    panic!("Expected token after operator to be a value or expression")
                }
            },
            right: match next_element {
                BinaryExpressionToken::Literal(val) => Expr::Literal(val.clone()),
                BinaryExpressionToken::Expr(expr) => expr,
                BinaryExpressionToken::Operator(_) => {
                    panic!("Expected token after operator to be a value or expression")
                }
            },
            operator: op,
        }))),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_binary_op() {
        assert_eq!(
            parse_binary_expression("1 * 2"),
            BinaryExpr {
                left: Expr::Literal(Literal::Int(1)),
                right: Expr::Literal(Literal::Int(2)),
                operator: BinaryOperator::Mul
            }
        );
        assert_eq!(
            parse_binary_expression("1 + 2"),
            BinaryExpr {
                left: Expr::Literal(Literal::Int(1)),
                right: Expr::Literal(Literal::Int(2)),
                operator: BinaryOperator::Add
            }
        );
        assert_eq!(
            parse_binary_expression("1 + 2 * 2"),
            BinaryExpr {
                left: Expr::Literal(Literal::Int(1)),
                right: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Literal(Literal::Int(2)),
                    right: Expr::Literal(Literal::Int(2)),
                    operator: BinaryOperator::Mul
                })),
                operator: BinaryOperator::Add
            }
        );
        assert_eq!(
            parse_binary_expression("1 + 2 * 2 / 2"),
            BinaryExpr {
                left: Expr::Literal(Literal::Int(1)),
                right: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::BinaryExpr(Box::new(BinaryExpr {
                        left: Expr::Literal(Literal::Int(2)),
                        right: Expr::Literal(Literal::Int(2)),
                        operator: BinaryOperator::Mul
                    })),
                    right: Expr::Literal(Literal::Int(2)),
                    operator: BinaryOperator::Div
                })),
                operator: BinaryOperator::Add
            }
        );
    }
    #[test]
    fn test_parse_binary_expr() {
        assert_eq!(
            parse_binary_expression("1 + 2"),
            BinaryExpr {
                left: Expr::Literal(Literal::Int(1)),
                right: Expr::Literal(Literal::Int(2)),
                operator: BinaryOperator::Add
            }
        );
        assert_eq!(
            parse_binary_expression("2 > 1"),
            BinaryExpr {
                left: Expr::Literal(Literal::Int(2)),
                right: Expr::Literal(Literal::Int(1)),
                operator: BinaryOperator::Gt
            }
        );
        assert_eq!(
            parse_binary_expression("2 == 2"),
            BinaryExpr {
                left: Expr::Literal(Literal::Int(2)),
                right: Expr::Literal(Literal::Int(2)),
                operator: BinaryOperator::Eq
            }
        );
        assert_eq!(
            parse_binary_expression("true || false"),
            BinaryExpr {
                left: Expr::Literal(Literal::Bool(true)),
                right: Expr::Literal(Literal::Bool(false)),
                operator: BinaryOperator::Or
            }
        );
        assert_eq!(
            parse_binary_expression("true && false"),
            BinaryExpr {
                left: Expr::Literal(Literal::Bool(true)),
                right: Expr::Literal(Literal::Bool(false)),
                operator: BinaryOperator::And
            }
        );
    }
    #[test]
    fn test_parse_sub_expressions() {
        assert_eq!(
            parse_binary_expression("1 + (2 + 1)"),
            BinaryExpr {
                left: Expr::Literal(Literal::Int(1)),
                right: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Literal(Literal::Int(2)),
                    right: Expr::Literal(Literal::Int(1)),
                    operator: BinaryOperator::Add
                })),
                operator: BinaryOperator::Add
            }
        );
        assert_eq!(
            parse_binary_expression("(1 + 2) + 1"),
            BinaryExpr {
                left: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Literal(Literal::Int(1)),
                    right: Expr::Literal(Literal::Int(2)),
                    operator: BinaryOperator::Add
                })),
                right: Expr::Literal(Literal::Int(1)),
                operator: BinaryOperator::Add
            }
        );
        assert_eq!(
            parse_binary_expression("(1 + 2 - (1 + 1)) + 1"),
            BinaryExpr {
                left: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::BinaryExpr(Box::new(BinaryExpr {
                        left: Expr::Literal(Literal::Int(1)),
                        right: Expr::Literal(Literal::Int(2)),
                        operator: BinaryOperator::Add
                    })),
                    right: Expr::BinaryExpr(Box::new(BinaryExpr {
                        left: Expr::Literal(Literal::Int(1)),
                        right: Expr::Literal(Literal::Int(1)),
                        operator: BinaryOperator::Add
                    })),
                    operator: BinaryOperator::Sub
                })),
                right: Expr::Literal(Literal::Int(1)),
                operator: BinaryOperator::Add
            }
        );
        assert_eq!(
            parse_binary_expression("1 + n"),
            BinaryExpr {
                left: Expr::Literal(Literal::Int(1)),
                operator: BinaryOperator::Add,
                right: Expr::Symbol(String::from("n"))
            }
        );
    }
}

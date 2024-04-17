use super::{binary_operator::parse_binary_operator, parse_expression};
use crate::{
    errors::ParsingResult, literal::parse_literal, utils::get_pair_location, DashlangParser, Rule,
};
use ast::{BinaryExpr, BinaryOperator, Expr, Literal, Location, Symbol};
use pest::Parser;

#[derive(Debug, Clone, PartialEq)]
enum BinaryExpressionToken {
    Literal(Literal),
    Expr(Expr),
    Operator(BinaryOperator),
}
pub fn parse_binary_expression(input: &str, base_location: usize) -> ParsingResult<BinaryExpr> {
    let ast = DashlangParser::parse(Rule::binary_expression, input)
        .expect("Could not parse binary expression")
        .next()
        .expect("Could not parse binary expression");
    let (start, end) = get_pair_location(&ast);
    let mut flat_expression: Vec<BinaryExpressionToken> = vec![];
    for element in ast.into_inner() {
        let (element_start, element_end) = get_pair_location(&element);
        match element.as_rule() {
            Rule::binary_operator => flat_expression.push(BinaryExpressionToken::Operator(
                parse_binary_operator(element.as_str()),
            )),
            Rule::literal => {
                flat_expression.push(BinaryExpressionToken::Literal(parse_literal(
                    element.as_str(),
                    element_start + base_location,
                )?));
            }
            Rule::expression => {
                let parsed = parse_expression(element.as_str(), element_start)?;
                flat_expression.push(BinaryExpressionToken::Expr(parsed));
            }
            Rule::symbol => {
                let parsed = element.as_str().to_owned();
                flat_expression.push(BinaryExpressionToken::Expr(Expr::Symbol(Symbol {
                    value: parsed,
                    location: Location::new(
                        element_start + base_location,
                        element_end + base_location,
                    ),
                })));
            }
            Rule::call_expression => {
                let parsed = parse_expression(element.as_str(), element_start + base_location)?;
                flat_expression.push(BinaryExpressionToken::Expr(parsed));
            }
            _ => unreachable!(),
        }
    }
    let mut base_binary_op = flat_binary_expression_to_ast(&mut flat_expression);
    base_binary_op.location = Location::new(start + base_location, end + base_location);
    Ok(base_binary_op)
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
        .iter_mut()
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
    let left = match previous_element {
        BinaryExpressionToken::Literal(val) => Expr::Literal(val.clone()),
        BinaryExpressionToken::Expr(expr) => expr,
        BinaryExpressionToken::Operator(_) => {
            panic!("Expected token after operator to be a value or expression")
        }
    };
    let right = match next_element {
        BinaryExpressionToken::Literal(val) => Expr::Literal(val.clone()),
        BinaryExpressionToken::Expr(expr) => expr,
        BinaryExpressionToken::Operator(_) => {
            panic!("Expected token after operator to be a value or expression")
        }
    };
    let _ = std::mem::replace(
        &mut flat_expression[operator_pos - 1], // Since we removed the previous item, we use position - 1
        BinaryExpressionToken::Expr(Expr::BinaryExpr(Box::new(BinaryExpr {
            location: Location::new(left.get_location().start, right.get_location().end),
            left,
            right,
            operator: op,
        }))),
    );
}

#[cfg(test)]
mod tests {
    use ast::{Boolean, Int};

    use super::*;

    #[test]
    fn test_parse_binary_op() {
        assert_eq!(
            parse_binary_expression("1 * 2", 0),
            Ok(BinaryExpr {
                left: Expr::Literal(Literal::Int(Int {
                    value: 1,
                    location: Location::new(0, 1)
                })),
                right: Expr::Literal(Literal::Int(Int {
                    value: 2,
                    location: Location::new(4, 5)
                })),
                operator: BinaryOperator::Mul,
                location: Location::new(0, 5),
            })
        );
        assert_eq!(
            parse_binary_expression("1 + 2", 0),
            Ok(BinaryExpr {
                left: Expr::Literal(Literal::Int(Int {
                    value: 1,
                    location: Location::new(0, 1)
                })),
                right: Expr::Literal(Literal::Int(Int {
                    value: 2,
                    location: Location::new(4, 5)
                })),
                operator: BinaryOperator::Add,
                location: Location::new(0, 5),
            })
        );
        assert_eq!(
            parse_binary_expression("1 + 2 * 2", 0),
            Ok(BinaryExpr {
                left: Expr::Literal(Literal::Int(Int {
                    value: 1,
                    location: Location::new(0, 1)
                })),
                right: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Literal(Literal::Int(Int {
                        value: 2,
                        location: Location::new(4, 5)
                    })),
                    right: Expr::Literal(Literal::Int(Int {
                        value: 2,
                        location: Location::new(8, 9)
                    })),
                    operator: BinaryOperator::Mul,
                    location: Location::new(4, 9),
                })),
                operator: BinaryOperator::Add,
                location: Location::new(0, 9),
            })
        );
    }
    #[test]
    fn test_parse_binary_expr() {
        assert_eq!(
            parse_binary_expression("1 + 2", 0),
            Ok(BinaryExpr {
                left: Expr::Literal(Literal::Int(Int {
                    value: 1,
                    location: Location::new(0, 1)
                })),
                right: Expr::Literal(Literal::Int(Int {
                    value: 2,
                    location: Location::new(4, 5)
                })),
                operator: BinaryOperator::Add,
                location: Location::new(0, 5),
            })
        );
        assert_eq!(
            parse_binary_expression("2 > 1", 0),
            Ok(BinaryExpr {
                left: Expr::Literal(Literal::Int(Int {
                    value: 2,
                    location: Location::new(0, 1)
                })),
                right: Expr::Literal(Literal::Int(Int {
                    value: 1,
                    location: Location::new(4, 5)
                })),
                operator: BinaryOperator::Gt,
                location: Location::new(0, 5),
            })
        );
        assert_eq!(
            parse_binary_expression("2 == 2", 0),
            Ok(BinaryExpr {
                left: Expr::Literal(Literal::Int(Int {
                    value: 2,
                    location: Location::new(0, 1)
                })),
                right: Expr::Literal(Literal::Int(Int {
                    value: 2,
                    location: Location::new(5, 6)
                })),
                operator: BinaryOperator::Eq,
                location: Location::new(0, 6),
            })
        );
        assert_eq!(
            parse_binary_expression("true || false", 0),
            Ok(BinaryExpr {
                left: Expr::Literal(Literal::Bool(Boolean {
                    value: true,
                    location: Location::new(0, 4)
                })),
                right: Expr::Literal(Literal::Bool(Boolean {
                    value: false,
                    location: Location::new(8, 13)
                })),
                operator: BinaryOperator::Or,
                location: Location::new(0, 13),
            })
        );
        assert_eq!(
            parse_binary_expression("true && false", 0),
            Ok(BinaryExpr {
                left: Expr::Literal(Literal::Bool(Boolean {
                    value: true,
                    location: Location::new(0, 4)
                })),
                right: Expr::Literal(Literal::Bool(Boolean {
                    value: false,
                    location: Location::new(8, 13)
                })),
                operator: BinaryOperator::And,
                location: Location::new(0, 13),
            })
        );
    }
    #[test]
    fn test_parse_sub_expressions() {
        assert_eq!(
            parse_binary_expression("1 + (2 + 1)", 0),
            Ok(BinaryExpr {
                left: Expr::Literal(Literal::Int(Int {
                    value: 1,
                    location: Location::new(0, 1)
                })),
                right: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Literal(Literal::Int(Int {
                        value: 2,
                        location: Location::new(5, 6)
                    })),
                    right: Expr::Literal(Literal::Int(Int {
                        value: 1,
                        location: Location::new(9, 10)
                    })),
                    operator: BinaryOperator::Add,
                    location: Location::new(5, 10),
                })),
                operator: BinaryOperator::Add,
                location: Location::new(0, 11),
            }),
        );
        assert_eq!(
            parse_binary_expression("(1 + 2) + 1", 0),
            Ok(BinaryExpr {
                left: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Literal(Literal::Int(Int {
                        value: 1,
                        location: Location::new(1, 2)
                    })),
                    right: Expr::Literal(Literal::Int(Int {
                        value: 2,
                        location: Location::new(5, 6)
                    })),
                    operator: BinaryOperator::Add,
                    location: Location::new(1, 6),
                })),
                right: Expr::Literal(Literal::Int(Int {
                    value: 1,
                    location: Location::new(10, 11)
                })),
                operator: BinaryOperator::Add,
                location: Location::new(0, 11),
            })
        );
        assert_eq!(
            parse_binary_expression("1 + n", 0),
            Ok(BinaryExpr {
                left: Expr::Literal(Literal::Int(Int {
                    value: 1,
                    location: Location::new(0, 1)
                })),
                operator: BinaryOperator::Add,
                right: Expr::Symbol(Symbol {
                    value: String::from("n"),
                    location: Location::new(4, 5)
                }),
                location: Location::new(0, 5),
            })
        );
    }
}

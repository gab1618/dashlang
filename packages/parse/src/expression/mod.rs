use crate::{literal::parse_literal, DashlangParser, Rule};
use ast::Expr;
use pest::Parser;

use self::{
    assignment_expression::parse_assignment_expression, binary_expression::parse_binary_expression,
    call_expression::parse_call_expression, compound_assign_expr::parse_compound_assign_expr,
    unary_expression::parse_unary_expression,
};

mod assignment_expression;
mod binary_expression;
mod binary_operator;
mod call_expression;
mod compound_assign_expr;
mod unary_expression;

pub fn parse_expression(input: &str) -> Expr {
    let ast = DashlangParser::parse(Rule::expression, input)
        .expect("Could not parse expression")
        .next()
        .expect("Could not parse expression");
    let mut inner_ast = ast.into_inner();
    let expression = inner_ast.next().expect("Could not get expression type");
    let mut parsed = match expression.as_rule() {
        Rule::binary_expression => {
            let parsed = parse_binary_expression(expression.as_str());
            Expr::BinaryExpr(Box::new(parsed))
        }
        Rule::assignment_expression => {
            let parsed = parse_assignment_expression(expression.as_str());
            Expr::Assignment(parsed)
        }
        Rule::compound_assignment_expr => {
            Expr::Assignment(parse_compound_assign_expr(expression.as_str()))
        }
        Rule::call_expression => {
            let parsed = parse_call_expression(expression.as_str());
            Expr::Call(parsed)
        }
        Rule::symbol => Expr::Symbol(expression.as_str().to_owned()),
        Rule::literal => Expr::Literal(parse_literal(expression.as_str())),
        Rule::unary_expression => {
            Expr::UnaryExpr(Box::new(parse_unary_expression(expression.as_str())))
        }
        any => unreachable!("{:#?}", any),
    };
    for piping in inner_ast {
        let inner_call = piping
            .into_inner()
            .next()
            .expect("Could not get call from piping");
        let mut parsed_inner_call = parse_call_expression(inner_call.as_str());
        parsed_inner_call.args.insert(0, parsed);
        parsed = Expr::Call(parsed_inner_call);
    }
    parsed
}
#[cfg(test)]
mod tests {
    use super::*;
    use ast::{
        AssignmentExpr, BinaryExpr, BinaryOperator, Boolean, Call, Expr, Int, Literal, Location,
        Str, UnaryExpr,
    };
    #[test]
    fn test_parse_expression() {
        assert_eq!(
            parse_expression("1 + 2"),
            Expr::BinaryExpr(Box::new(BinaryExpr {
                left: Expr::Literal(Literal::Int(Int {
                    value: 1,
                    location: Location::new(0, 1)
                })),
                right: Expr::Literal(Literal::Int(Int {
                    value: 2,
                    location: Location::new(0, 1)
                })),
                operator: BinaryOperator::Add,
                location: Location::default(),
            }))
        );
    }
    #[test]
    fn test_assignment_expression() {
        assert_eq!(
            parse_expression("age = 5 + 1"),
            Expr::Assignment(AssignmentExpr {
                symbol: String::from("age"),
                value: Box::new(Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Literal(Literal::Int(Int {
                        value: 5,
                        location: Location::new(0, 1)
                    })),
                    right: Expr::Literal(Literal::Int(Int {
                        value: 1,
                        location: Location::new(0, 1)
                    })),
                    operator: BinaryOperator::Add,
                    location: Location::default(),
                }))),
                location: Location::default(),
            })
        );
    }
    #[test]
    fn test_unary_expression() {
        assert_eq!(
            parse_expression("!(true && false)"),
            Expr::UnaryExpr(Box::new(UnaryExpr {
                operator: ast::UnaryOperator::Not,
                operand: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Literal(Literal::Bool(Boolean {
                        value: true,
                        location: Location::new(0, 4)
                    })),
                    right: Expr::Literal(Literal::Bool(Boolean {
                        value: false,
                        location: Location::new(0, 5)
                    })),
                    operator: BinaryOperator::And,
                    location: Location::default(),
                })),
                location: Location::new(0, 16),
            }))
        );
    }
    #[test]
    fn test_compound_assign_expr() {
        assert_eq!(
            parse_expression("n += 1"),
            Expr::Assignment(AssignmentExpr {
                symbol: String::from("n"),
                value: Box::new(Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Symbol(String::from("n")),
                    right: Expr::Literal(Literal::Int(Int {
                        value: 1,
                        location: Location::new(0, 1)
                    })),
                    operator: BinaryOperator::Add,
                    location: Location::default(),
                }))),
                location: Location::default(),
            })
        );
    }
    #[test]
    fn test_piping() {
        assert_eq!(
            parse_expression("4 |> add(1)"),
            Expr::Call(Call {
                symbol: String::from("add"),
                args: vec![
                    Expr::Literal(Literal::Int(Int {
                        value: 4,
                        location: Location::new(0, 1)
                    })),
                    Expr::Literal(Literal::Int(Int {
                        value: 1,
                        location: Location::new(0, 1)
                    }))
                ],
                location: Location::default(),
            })
        );
        assert_eq!(
            parse_expression("4 |> add(1) |> add(5)"),
            Expr::Call(Call {
                symbol: String::from("add"),
                args: vec![
                    Expr::Call(Call {
                        symbol: String::from("add"),
                        args: vec![
                            Expr::Literal(Literal::Int(Int {
                                value: 4,
                                location: Location::new(0, 1)
                            })),
                            Expr::Literal(Literal::Int(Int {
                                value: 1,
                                location: Location::new(0, 1)
                            }))
                        ],
                        location: Location::default(),
                    }),
                    Expr::Literal(Literal::Int(Int {
                        value: 5,
                        location: Location::new(0, 1)
                    }))
                ],
                location: Location::default(),
            })
        );
        assert_eq!(
            parse_expression("\"Hello, \" |> push(\"World!\")"),
            Expr::Call(Call {
                symbol: String::from("push"),
                args: vec![
                    Expr::Literal(Literal::String(Str {
                        value: String::from("Hello, "),
                        location: Location::new(0, 9)
                    })),
                    Expr::Literal(Literal::String(Str {
                        value: String::from("World!"),
                        location: Location::new(0, 8)
                    }))
                ],
                location: Location::default(),
            })
        );
    }
}

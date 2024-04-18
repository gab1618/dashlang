use ast::{For, Location};
use errors::DashlangResult;
use pest::Parser;

use crate::{
    body::parse_body,
    expression::parse_expression,
    parser::{DashlangParser, Rule},
    utils::get_pair_location,
};

use super::parse_statement;

pub fn parse_for_stmt(input: &str, base_location: usize) -> DashlangResult<For> {
    let ast = DashlangParser::parse(Rule::for_stmt, input)
        .expect("Could not parse for statement")
        .next()
        .expect("Could not get for statement");
    let (start, end) = get_pair_location(&ast);
    let mut inner_ast = ast.into_inner();
    let init_stmt = inner_ast
        .next()
        .expect("Could not get init statement from for statement");
    let (init_start, _) = get_pair_location(&init_stmt);
    let cond_expr = inner_ast
        .next()
        .expect("Could not get condition expr from for statement");
    let (cond_start, _) = get_pair_location(&cond_expr);
    let iteration_statement = inner_ast
        .next()
        .expect("Could not get iteration statement from for statement");
    let (iteration_start, _) = get_pair_location(&iteration_statement);
    let for_body = inner_ast.next().expect("Could not get for statement body");
    let (body_start, _) = get_pair_location(&for_body);

    Ok(For {
        cond: parse_expression(cond_expr.as_str(), cond_start + base_location)?,
        body: parse_body(for_body.as_str(), body_start + base_location)?,
        init: parse_statement(init_stmt.as_str(), init_start + base_location)?,
        iteration: parse_statement(
            iteration_statement.as_str(),
            iteration_start + base_location,
        )?,
        location: Location::new(start, end),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::{
        AssignmentExpr, BinaryExpr, BinaryOperator, Expr, Int, Literal, Location, Stmt, Symbol,
    };
    #[test]
    fn test_for_stmt() {
        assert_eq!(
            parse_for_stmt("for n = 1; n < 10; n += 1 {}", 0),
            Ok(For {
                init: Stmt::Expr(Expr::Assignment(AssignmentExpr {
                    symbol: String::from("n"),
                    value: Box::new(Expr::Literal(Literal::Int(Int {
                        value: 1,
                        location: Location::new(8, 9)
                    }))),
                    location: Location::new(4, 9),
                })),
                cond: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Symbol(Symbol {
                        value: String::from("n"),
                        location: Location::new(11, 12)
                    }),
                    right: Expr::Literal(Literal::Int(Int {
                        value: 10,
                        location: Location::new(15, 17)
                    })),
                    operator: BinaryOperator::Lt,
                    location: Location::new(11, 17),
                })),
                body: vec![],
                iteration: Stmt::Expr(Expr::Assignment(AssignmentExpr {
                    symbol: String::from("n"),
                    value: Box::new(Expr::BinaryExpr(Box::new(BinaryExpr {
                        left: Expr::Symbol(Symbol {
                            value: String::from("n"),
                            location: Location::new(19, 20)
                        }),
                        right: Expr::Literal(Literal::Int(Int {
                            value: 1,
                            location: Location::new(24, 25)
                        })),
                        operator: BinaryOperator::Add,
                        location: Location::new(19, 26),
                    }))),
                    location: Location::new(19, 26),
                })),
                location: Location::new(0, 28),
            })
        );
    }
}

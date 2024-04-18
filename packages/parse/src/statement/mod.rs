mod for_stmt;
mod if_stmt;
mod return_stmt;
mod while_stmt;
use ast::Stmt;
use errors::DashlangResult;
use pest::Parser;

use crate::{
    parser::{DashlangParser, Rule},
    utils::get_pair_location,
};

use return_stmt::parse_return_stmt;

use self::{for_stmt::parse_for_stmt, if_stmt::parse_if_stmt, while_stmt::parse_while_stmt};

pub fn parse_statement(input: &str, base_location: usize) -> DashlangResult<Stmt> {
    let ast = DashlangParser::parse(Rule::statement, input)
        .expect("Could not parse statement")
        .next()
        .expect("Could not parse statement");
    let ast_statement = ast.into_inner().next().expect("Could not get statement");
    let (statement_start, _) = get_pair_location(&ast_statement);
    Ok(match ast_statement.as_rule() {
        Rule::return_stmt => {
            parse_return_stmt(ast_statement.as_str(), statement_start + base_location)?
        }
        Rule::if_stmt => Stmt::If(parse_if_stmt(
            ast_statement.as_str(),
            statement_start + base_location,
        )?),
        Rule::while_stmt => Stmt::While(parse_while_stmt(
            ast_statement.as_str(),
            statement_start + base_location,
        )?),
        Rule::for_stmt => Stmt::For(Box::new(parse_for_stmt(
            ast_statement.as_str(),
            statement_start + base_location,
        )?)),
        _ => unreachable!(),
    })
}
#[cfg(test)]
mod tests {
    use ast::{
        BinaryExpr, BinaryOperator, Expr, If, Int, Literal, Location, Return, Stmt, Symbol, While,
    };

    use super::*;
    #[test]
    fn test_parse_ret_stmt() {
        assert_eq!(
            parse_statement("return 5", 0),
            Ok(Stmt::Return(Return {
                value: Expr::Literal(Literal::Int(Int {
                    value: 5,
                    location: Location::new(7, 8)
                })),
                location: Location::new(0, 8)
            }))
        );
    }
    #[test]
    fn test_parse_if() {
        assert_eq!(
            parse_statement("if count < 5 {}", 0),
            Ok(Stmt::If(If {
                cond: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Symbol(Symbol {
                        value: String::from("count"),
                        location: Location::new(3, 8)
                    }),
                    right: Expr::Literal(Literal::Int(Int {
                        value: 5,
                        location: Location::new(11, 12)
                    })),
                    operator: BinaryOperator::Lt,
                    location: Location::new(3, 13),
                })),
                else_block: None,
                body: vec![],
                location: Location::new(0, 15),
            }))
        );
    }
    #[test]
    fn test_parse_while() {
        assert_eq!(
            parse_statement("while count < 5 {}", 0),
            Ok(Stmt::While(While {
                cond: Expr::BinaryExpr(Box::new(BinaryExpr {
                    left: Expr::Symbol(Symbol {
                        value: String::from("count"),
                        location: Location::new(6, 11)
                    }),
                    right: Expr::Literal(Literal::Int(Int {
                        value: 5,
                        location: Location::new(14, 15)
                    })),
                    operator: BinaryOperator::Lt,
                    location: Location::new(6, 16),
                })),
                body: vec![],
                location: Location::new(0, 18),
            }))
        );
    }
}

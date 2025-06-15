use ast::{AssignmentExpr, Expr};
use errors::{DashlangError, DashlangResult, ErrorKind};
use pest::Parser;

use crate::{
    expression::{call_expression::parse_call_expression, parse_expression},
    parser::{DashlangParser, Rule},
    utils::get_pair_location,
};

pub fn parse_dash_expression(input: &str, base_location: usize) -> DashlangResult<AssignmentExpr> {
    let ast = DashlangParser::parse(Rule::dash_expr, input)
        .map_err(|err| {
            DashlangError::new("Could not parse dash expression", ErrorKind::Unknown).location(
                match err.location {
                    pest::error::InputLocation::Pos(pos) => (pos, pos + 1).into(),
                    pest::error::InputLocation::Span((start, end)) => (start, end).into(),
                },
            )
        })?
        .next()
        .ok_or(DashlangError::new(
            "Could not parse dash expression",
            ErrorKind::Unknown,
        ))?;
    let (ast_start, ast_end) = get_pair_location(&ast);
    let mut ast_inner = ast.into_inner();
    let ast_symbol = ast_inner.next().unwrap();
    let (symbol_start, _) = get_pair_location(&ast_symbol);
    let parsed_symbol = parse_expression(ast_symbol.as_str(), symbol_start + base_location)?;

    let ast_call = ast_inner.next().unwrap();
    let (call_start, _) = get_pair_location(&ast_call);
    let mut parsed_call = parse_call_expression(ast_call.as_str(), call_start + base_location)?;
    parsed_call.args.insert(0, parsed_symbol);
    Ok(AssignmentExpr {
        symbol: ast_symbol.as_str().to_owned(),
        value: Box::new(Expr::Call(parsed_call)),
        location: (ast_start + base_location, ast_end + base_location).into(),
    })
}

#[cfg(test)]
mod tests {
    use ast::{Call, Expr, Symbol};

    use super::*;

    #[test]
    fn test_parse_simple_dash_expr() {
        assert_eq!(
            parse_dash_expression("n |>= twice()", 0),
            Ok(AssignmentExpr {
                symbol: "n".to_owned(),
                value: Box::new(Expr::Call(Call {
                    symbol: "twice".to_owned(),
                    args: vec![Expr::Symbol(Symbol {
                        value: "n".to_owned(),
                        location: (0, 1).into()
                    })],
                    location: (6, 13).into()
                })),
                location: (0, 13).into()
            })
        );
    }
}

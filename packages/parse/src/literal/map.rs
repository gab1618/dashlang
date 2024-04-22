use std::collections::HashMap;

use ast::Map;
use errors::DashlangResult;
use pest::Parser;

use crate::{
    expression::parse_expression,
    parser::{DashlangParser, Rule},
    utils::get_pair_location,
};

pub fn parse_map(input: &str, base_location: usize) -> DashlangResult<Map> {
    let ast = DashlangParser::parse(Rule::map, input)
        .expect("Could not parse map")
        .next()
        .expect("Could not parse map");
    let (start, end) = get_pair_location(&ast);
    let mut map_value = HashMap::new();
    for attribute in ast.into_inner() {
        let mut inner_attribute = attribute.into_inner();
        let attr_ast_symbol = inner_attribute.next().expect("Could not parse attribute");

        let attr_ast_value = inner_attribute
            .next()
            .expect("Could not get attribute valie");
        let (value_start, _) = get_pair_location(&attr_ast_value);
        let parsed_attr_value =
            parse_expression(attr_ast_value.as_str(), base_location + value_start)?;
        let parsed_ast_symbol = attr_ast_symbol.as_str().to_owned();

        map_value.insert(parsed_ast_symbol, parsed_attr_value);
    }
    Ok(Map {
        value: map_value,
        location: (start + base_location, end + base_location).into(),
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use ast::{Expr, Int, Literal};

    use super::*;
    #[test]
    fn test_parse_map() {
        assert_eq!(
            parse_map("{count: 0, count2: 1}", 0),
            Ok(Map {
                value: HashMap::from([
                    (
                        "count".to_owned(),
                        Expr::Literal(Literal::Int(Int {
                            value: 0,
                            location: (8, 9).into()
                        }))
                    ),
                    (
                        "count2".to_owned(),
                        Expr::Literal(Literal::Int(Int {
                            value: 1,
                            location: (19, 20).into()
                        }))
                    )
                ]),
                location: (0, 21).into()
            })
        );
    }
}

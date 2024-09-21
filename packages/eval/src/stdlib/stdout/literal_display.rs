use ast::Literal;
use errors::{DashlangError, DashlangResult};

use crate::{ctx::Context, eval, scope::Scope};

pub fn stdlib_literal_display<T: Scope + Clone>(
    value: &Literal,
    ctx: &Context<T>,
) -> Result<String, DashlangError> {
    match value {
        Literal::Closure(_) => Ok("Closure".to_string()),
        Literal::Int(val) => Ok(format!("{}", val.value)),
        Literal::Float(val) => Ok(format!("{}", val.value)),
        Literal::String(val) => Ok(val.clone().value),
        Literal::Bool(val) => Ok(if val.value {
            "True".to_string()
        } else {
            "False".to_string()
        }),
        Literal::Vector(val) => {
            let display_args: Result<Vec<String>, DashlangError> = val
                .clone()
                .value
                .into_iter()
                .map(|item| stdlib_literal_display(&eval(item.clone(), ctx)?, ctx))
                .collect();
            match display_args {
                Ok(args) => Ok(format!("[{}]", args.join(", "))),
                Err(err) => Err(err),
            }
        }
        Literal::Null(_) => Ok("Null".to_string()),
        Literal::Void(_) => Ok("Void".to_string()),
        Literal::Tuple(tup) => {
            let display_values: DashlangResult<Vec<String>> = tup
                .clone()
                .value
                .into_iter()
                .map(|item| stdlib_literal_display(&eval(item.clone(), ctx)?, ctx))
                .collect();
            match display_values {
                Ok(args) => Ok(format!("({})", args.join(", "))),
                Err(err) => Err(err),
            }
        }
        Literal::Map(map) => {
            let mut formated_attributes: Vec<String> = vec![];
            for (symbol, value) in map.value.iter() {
                formated_attributes.push(format!(
                    "{symbol}: {}",
                    stdlib_literal_display(&eval(value.clone(), ctx)?, ctx)?
                ));
            }
            Ok(format!("{{ {} }}", formated_attributes.join(", ")))
        }
        Literal::Atom(atom) => Ok(format!(":{}", atom.value)),
    }
}

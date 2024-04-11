use ast::Literal;

use crate::{errors::RuntimeError, eval, scope::Scope, Context, RuntimeResult};

pub fn stdlib_nth<T: Scope + Clone>(
    value: Literal,
    index: Literal,
    ctx: &Context<T>,
) -> RuntimeResult<Literal> {
    if let Literal::Int(int_index) = index {
        if let Literal::Vector(vec) = value {
            if (int_index.value as usize) < vec.value.len() {
                return eval(vec.value[int_index.value as usize].clone(), ctx);
            }
            return Err(RuntimeError::new("Index out of bound"));
        }
        return Err(RuntimeError::new("Expected vector to be indexed"));
    }
    Err(RuntimeError::new("Expected integer to index vector"))
}

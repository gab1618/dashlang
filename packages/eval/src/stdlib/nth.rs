use ast::Literal;

use crate::{eval, scope::Scope, Context, RuntimeResult};

pub fn stdlib_nth<T: Scope + Clone>(
    value: Literal,
    index: Literal,
    ctx: &Context<T>,
) -> RuntimeResult<Literal> {
    if let Literal::Int(int_index) = index {
        if let Literal::Vector(vec) = value {
            if (int_index as usize) < vec.len() {
                return eval(vec[int_index as usize].clone(), ctx);
            }
            panic!("Cannot index vector: index out of bounds")
        }
        panic!("Expected vector to be indexed")
    }
    panic!("Expected integer to index vector")
}

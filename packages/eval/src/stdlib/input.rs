use std::io;

use ast::Literal;

pub fn stdlib_input() -> Literal {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");
    Literal::String(input)
}

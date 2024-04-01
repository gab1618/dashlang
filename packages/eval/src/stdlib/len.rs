use ast::Literal;

pub fn stdlib_len(item: Literal) -> Literal {
    match item {
        Literal::String(val) => Literal::Int(val.len() as i64),
        Literal::Vector(val) => Literal::Int(val.len() as i64),
        _ => panic!("Unsuported operation"),
    }
}

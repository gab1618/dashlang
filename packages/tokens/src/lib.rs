pub struct AsignmentExpr {
    symbol: String,
    value: Box<Expr>,
}
pub enum BinaryOpType {
    Add,
    Sub,
    Mul,
    Div,
    Gt, // Greater than
    Eq, // Equal
    Ge, // Greater or equal than
    Lt, // Less than
    Le, // Less of equal than
}
pub struct BinaryOp {
    left: Expr,
    right: Expr,
    op_type: BinaryOpType,
}
pub struct Call {
    symbol: String,
    args: Vec<Expr>,
}
pub struct Closure {
    symbol: String,
    params: Vec<Expr>,
    body: Vec<Expr>,
}
#[derive(Debug, PartialEq)]
pub enum Primitive {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}
pub enum Expr {
    BinaryOp(Box<BinaryOp>),
    AsignmentExpr(AsignmentExpr),
    Closure(Closure),
    Call(Call),
    Primitive(Primitive),
    Symbol(String),
    Void,
}

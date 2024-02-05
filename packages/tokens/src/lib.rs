#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct AsignmentExpr {
    pub symbol: String,
    pub value: Box<Expr>,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
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
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct BinaryOp {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
    pub op_type: BinaryOpType,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Call {
    symbol: String,
    args: Vec<Expr>,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Closure {
    symbol: String,
    params: Vec<Expr>,
    body: Vec<Expr>,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Expr {
    BinaryOp(BinaryOp),
    AsignmentExpr(AsignmentExpr),
    Closure(Closure),
    Call(Call),
    Symbol(String),
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
}

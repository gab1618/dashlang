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
    And,
    Or,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct BinaryOp {
    pub left: Expr,
    pub right: Expr,
    pub op_type: BinaryOpType,
}
impl BinaryOp {
    pub fn new(left: Expr, right: Expr, op_type: BinaryOpType) -> Self {
        Self {
            left,
            right,
            op_type,
        }
    }
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
    BinaryOp(Box<BinaryOp>),
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

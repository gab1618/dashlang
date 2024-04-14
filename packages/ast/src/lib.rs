#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}
impl Location {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}
impl Default for Location {
    fn default() -> Self {
        Location::new(0, 0)
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct AssignmentExpr {
    pub symbol: String,
    pub value: Box<Expr>,
    pub location: Location,
}
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum BinaryOperator {
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
pub struct BinaryExpr {
    pub left: Expr,
    pub right: Expr,
    pub operator: BinaryOperator,
    pub location: Location,
}
impl BinaryExpr {
    pub fn new(left: Expr, right: Expr, op_type: BinaryOperator) -> Self {
        Self {
            left,
            right,
            operator: op_type,
            location: Location::new(0, 0),
        }
    }
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum UnaryOperator {
    Not,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub operand: Expr,
    pub location: Location,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Call {
    pub symbol: String,
    pub args: Vec<Expr>,
    pub location: Location,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Closure {
    pub params: Vec<String>,
    pub body: Program,
    pub location: Location,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Symbol {
    pub value: String,
    pub location: Location,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Expr {
    BinaryExpr(Box<BinaryExpr>),
    UnaryExpr(Box<UnaryExpr>),
    Assignment(AssignmentExpr),
    Call(Call),
    Symbol(Symbol),
    Literal(Literal),
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Int {
    pub value: i64,
    pub location: Location,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Float {
    pub value: f64,
    pub location: Location,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Str {
    pub value: String,
    pub location: Location,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Boolean {
    pub value: bool,
    pub location: Location,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Vector {
    pub value: Vec<Expr>,
    pub location: Location,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Null {
    pub location: Location,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Void {
    pub location: Location,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Literal {
    Closure(Closure),
    Int(Int),
    Float(Float),
    String(Str),
    Bool(Boolean),
    Vector(Vector),
    Null(Null),
    Void(Void),
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct If {
    pub cond: Expr,
    pub body: Program,
    pub else_block: Option<Program>,
    pub location: Location,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct While {
    pub cond: Expr,
    pub body: Program,
    pub location: Location,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct For {
    pub cond: Expr,
    pub body: Program,
    pub init: Instruction,
    pub iteration: Instruction,
    pub location: Location,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Return {
    pub value: Expr,
    pub location: Location,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Stmt {
    Return(Return),
    If(If),
    While(While),
    For(Box<For>),
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Instruction {
    Stmt(Stmt),
    Expr(Expr),
}
pub type Program = Vec<Instruction>;

impl Expr {
    pub fn get_location(&self) -> Location {
        match self {
            Expr::BinaryExpr(val) => val.location,
            Expr::UnaryExpr(val) => val.location,
            Expr::Assignment(val) => val.location,
            Expr::Call(val) => val.location,
            Expr::Symbol(val) => val.location,
            Expr::Literal(val) => match val {
                Literal::Closure(lit) => lit.location,
                Literal::Int(lit) => lit.location,
                Literal::Float(lit) => lit.location,
                Literal::String(lit) => lit.location,
                Literal::Bool(lit) => lit.location,
                Literal::Vector(lit) => lit.location,
                Literal::Null(lit) => lit.location,
                Literal::Void(lit) => lit.location,
            },
        }
    }
}

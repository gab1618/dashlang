use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}
impl From<(usize, usize)> for Location {
    fn from(value: (usize, usize)) -> Self {
        let (start, end) = value;
        Location::new(start, end)
    }
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
#[derive(Debug, PartialEq, Clone)]
pub struct AssignmentExpr {
    pub symbol: String,
    pub value: Box<Expr>,
    pub location: Location,
}
#[derive(Debug, PartialEq, Clone, Copy)]
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
    BitwiseOr,
    BitwiseAnd,
    BitwiseXor,
    BitwiseShiftLeft,
    BitwiseShiftRight,
}
#[derive(Debug, PartialEq, Clone)]
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
    pub fn get_location(&self) -> Location {
        self.location
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Not,
    BitwiseNot,
}
#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpr {
    pub operator: UnaryOperator,
    pub operand: Expr,
    pub location: Location,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Call {
    pub symbol: String,
    pub args: Vec<Expr>,
    pub location: Location,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Closure {
    pub params: Vec<String>,
    pub body: Program,
    pub location: Location,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Symbol {
    pub value: String,
    pub location: Location,
}
#[derive(Debug, PartialEq, Clone)]
pub struct SubExpr {
    pub value: Box<Expr>,
    pub location: Location,
}
#[derive(Debug, PartialEq, Clone)]
pub struct DestructuringAsignment {
    pub location: Location,
    pub symbols: Vec<Symbol>,
    pub value: Box<Expr>,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    BinaryExpr(Box<BinaryExpr>),
    UnaryExpr(Box<UnaryExpr>),
    Assignment(AssignmentExpr),
    Call(Call),
    Symbol(Symbol),
    Literal(Literal),
    SubExpr(SubExpr),
    DestructuringAsignment(DestructuringAsignment),
}
impl Expr {
    pub fn get_location(&self) -> Location {
        match self {
            Expr::BinaryExpr(val) => val.location,
            Expr::UnaryExpr(val) => val.location,
            Expr::Assignment(val) => val.location,
            Expr::Call(val) => val.location,
            Expr::Symbol(val) => val.location,
            Expr::Literal(val) => val.get_location(),
            Expr::SubExpr(val) => val.location,
            Expr::DestructuringAsignment(val) => val.location,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Int {
    pub value: i64,
    pub location: Location,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Float {
    pub value: f64,
    pub location: Location,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Str {
    pub value: String,
    pub location: Location,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Boolean {
    pub value: bool,
    pub location: Location,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Vector {
    pub value: Vec<Expr>,
    pub location: Location,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Null {
    pub location: Location,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Void {
    pub location: Location,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Tuple {
    pub value: Vec<Expr>,
    pub location: Location,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Map {
    pub value: HashMap<String, Expr>,
    pub location: Location,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Atom {
    pub value: String,
    pub location: Location,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Closure(Closure),
    Int(Int),
    Float(Float),
    String(Str),
    Bool(Boolean),
    Vector(Vector),
    Null(Null),
    Void(Void),
    Tuple(Tuple),
    Map(Map),
    Atom(Atom),
}
impl Literal {
    pub fn get_location(&self) -> Location {
        match self {
            Literal::Closure(val) => val.location,
            Literal::Int(val) => val.location,
            Literal::Float(val) => val.location,
            Literal::String(val) => val.location,
            Literal::Bool(val) => val.location,
            Literal::Vector(val) => val.location,
            Literal::Null(val) => val.location,
            Literal::Void(val) => val.location,
            Literal::Tuple(val) => val.location,
            Literal::Map(val) => val.location,
            Literal::Atom(val) => val.location,
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct If {
    pub cond: Expr,
    pub body: Program,
    pub else_block: Option<Program>,
    pub location: Location,
}
#[derive(Debug, PartialEq, Clone)]
pub struct While {
    pub cond: Expr,
    pub body: Program,
    pub location: Location,
}
#[derive(Debug, PartialEq, Clone)]
pub struct For {
    pub cond: Expr,
    pub body: Program,
    pub init: Stmt,
    pub iteration: Stmt,
    pub location: Location,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Return {
    pub value: Expr,
    pub location: Location,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Return(Return),
    If(If),
    While(While),
    For(Box<For>),
    Expr(Expr),
}

pub type Program = Vec<Stmt>;

pub mod scope;
#[cfg(test)]
mod tests;

use std::{collections::HashMap, rc::Rc};

use ast::{BinaryExpr, BinaryOperator, Call, Expr, Instruction, Literal, Program, Stmt, UnaryExpr};
use scope::Scope;

macro_rules! define_aritmetic_operation {
    ($operator:tt, $op:expr, $scope:expr) => {
        match ($op.left, $op.right) {
            (Expr::Literal(left), Expr::Literal(right)) => match (left, right) {
                (Literal::Int(left), Literal::Int(right)) => Literal::Int(left $operator right),
                (Literal::Float(left), Literal::Int(right)) => Literal::Float(left $operator (right as f64)),
                (Literal::Int(left), Literal::Float(right)) => Literal::Float((left as f64) $operator right),
                (Literal::Float(left), Literal::Float(right)) => Literal::Float(left $operator right),
                (_, _) => panic!("Unsuported operation"),
            },
            (left, right) => eval_binary_op(
                BinaryExpr::new(
                    Expr::Literal(eval(left, $scope)),
                    Expr::Literal(eval(right, $scope)),
                    $op.operator,
                ),
                $scope,
            ),
        }
    };
}

macro_rules! define_boolean_operation {
    ($operator:tt, $op:expr, $scope:expr) => {
        match ($op.left, $op.right) {
            (Expr::Literal(left), Expr::Literal(right)) => match (left, right) {
                (Literal::Int(left), Literal::Int(right)) => Literal::Bool(left $operator right),
                (Literal::Float(left), Literal::Int(right)) => Literal::Bool(left $operator (right as f64)),
                (Literal::Int(left), Literal::Float(right)) => Literal::Bool((left as f64) $operator right),
                (Literal::Float(left), Literal::Float(right)) => Literal::Bool(left $operator right),
                (_, _) => panic!("Unsuported operation"),
            },
            (left, right) => eval_binary_op(
                BinaryExpr::new(
                    Expr::Literal(eval(left, $scope)),
                    Expr::Literal(eval(right, $scope)),
                    $op.operator,
                ),
                $scope,
            ),
        }
    };
}

fn is_truthy<T: Scope + Clone>(expr: Expr, scope: &Context<T>) -> bool {
    match expr {
        Expr::Literal(value) => match value {
            Literal::Closure(_) => true,
            Literal::Int(num) => num != 0,
            Literal::Float(num) => num != 0.0,
            Literal::String(string) => !string.is_empty(),
            Literal::Bool(bool) => bool,
            Literal::Null => false,
            Literal::Void => false,
        },
        expr => is_truthy(Expr::Literal(eval(expr, scope)), scope),
    }
}

fn eval_binary_op<T: Scope + Clone>(op: BinaryExpr, ctx: &Context<T>) -> Literal {
    match op.operator {
        BinaryOperator::Add => define_aritmetic_operation!(+, op, ctx),
        BinaryOperator::Sub => define_aritmetic_operation!(-, op, ctx),
        BinaryOperator::Mul => define_aritmetic_operation!(*, op, ctx),
        BinaryOperator::Div => define_aritmetic_operation!(/, op, ctx),
        BinaryOperator::Gt => define_boolean_operation!(>, op, ctx),
        BinaryOperator::Eq => define_boolean_operation!(==, op, ctx),
        BinaryOperator::Ge => define_boolean_operation!(>=, op, ctx),
        BinaryOperator::Lt => define_boolean_operation!(<, op, ctx),
        BinaryOperator::Le => define_boolean_operation!(<=, op, ctx),
        BinaryOperator::And => Literal::Bool(is_truthy(op.left, ctx) && is_truthy(op.right, ctx)),
        BinaryOperator::Or => Literal::Bool(is_truthy(op.left, ctx) || is_truthy(op.right, ctx)),
    }
}
fn eval_unary_op<T: Scope + Clone>(op: UnaryExpr, ctx: &Context<T>) -> Literal {
    match op.operator {
        ast::UnaryOperator::Not => match op.operand {
            Expr::Literal(literal) => Literal::Bool(!is_truthy(Expr::Literal(literal), ctx)),
            expr => {
                let literal_from_expr = eval(expr, ctx);
                let new_unary_op = UnaryExpr {
                    operator: ast::UnaryOperator::Not,
                    operand: Expr::Literal(literal_from_expr),
                };
                eval_unary_op(new_unary_op, ctx)
            }
        },
    }
}

pub fn eval_program<T: Scope + Clone>(program: Program, ctx: &Context<T>) -> Literal {
    for instruction in program {
        match instruction {
            Instruction::Stmt(stmt) => match stmt {
                Stmt::Return(val) => {
                    return eval(val, ctx);
                }
                Stmt::If(if_stmt) => {
                    if is_truthy(if_stmt.cond, ctx) {
                        let block_result = eval_program(if_stmt.body, ctx);
                        match block_result {
                            Literal::Void => (),
                            val => return val,
                        }
                    } else if let Some(else_block) = if_stmt.else_block {
                        let block_result = eval_program(else_block, ctx);
                        match block_result {
                            Literal::Null => (),
                            val => return val,
                        }
                    }
                }
                Stmt::While(while_stmt) => {
                    while is_truthy(while_stmt.clone().cond, ctx) {
                        let block_result = eval_program(while_stmt.clone().body, ctx);
                        match block_result {
                            Literal::Void => (),
                            val => return val,
                        }
                    }
                }
                Stmt::Print(expr) => {
                    let val = eval(expr, ctx);
                    match val {
                        Literal::Closure(_) => println!("[closure]"),
                        Literal::Int(num) => println!("{num}"),
                        Literal::Float(num) => println!("{num}"),
                        Literal::String(string) => println!("{string}"),
                        Literal::Bool(bool) => {
                            if bool {
                                println!("True")
                            } else {
                                println!("False")
                            }
                        }
                        Literal::Null => println!("null"),
                        Literal::Void => (),
                    }
                }
            },
            Instruction::Expr(expr) => {
                eval(expr, ctx);
            }
        }
    }
    Literal::Void
}

fn eval_call<T: Scope + Clone>(call: Call, ctx: &Context<T>) -> Literal {
    if let Some(found_extension) = ctx.extensions.get(&call.symbol) {
        let local_context = ctx.clone();
        let args: Vec<Literal> = call
            .args
            .into_iter()
            .map(|expr| eval(expr, &local_context))
            .collect();
        for (symbol, val) in found_extension.params.iter().zip(args) {
            // Inject all arguments into local scope
            local_context.scope.set(symbol, val);
        }
        return (found_extension.implementation)(&ctx);
    }
    if let Literal::Closure(closure) = ctx.scope.get(&call.symbol) {
        let local_context = ctx.clone();
        let args: Vec<Literal> = call
            .args
            .into_iter()
            .map(|expr| eval(expr, &local_context))
            .collect();
        for (symbol, val) in closure.params.into_iter().zip(args) {
            // Inject all arguments into local scope
            local_context.scope.set(&symbol, val);
        }
        eval_program(closure.body, &local_context)
    } else {
        panic!("Cannot call {}: not callable", call.symbol);
    }
}

pub fn eval<T: Scope + Clone>(expr: Expr, ctx: &Context<T>) -> Literal {
    match expr {
        Expr::Literal(val) => val,
        Expr::BinaryExpr(op) => eval_binary_op(*op, &ctx),
        Expr::Asignment(asign) => {
            let evaluated = eval(*asign.value, &ctx);
            ctx.scope.set(&asign.symbol, evaluated.clone());
            evaluated
        }
        Expr::Call(call) => eval_call(call, &ctx),
        Expr::Symbol(symbol) => ctx.scope.get(&symbol),
        Expr::UnaryExpr(op) => eval_unary_op(*op, &ctx),
    }
}
#[derive(Clone)]
pub struct Extension<S: Scope> {
    pub params: Vec<String>,
    pub implementation: Rc<dyn Fn(&Context<S>) -> Literal>,
}
pub struct Context<T: Scope> {
    scope: T,
    extensions: HashMap<String, Extension<T>>,
}
impl<T: Scope + Clone> Context<T> {
    pub fn new(s: T) -> Self {
        Self {
            scope: s,
            extensions: HashMap::new(),
        }
    }
    pub fn use_extension(mut self, extension: Extension<T>, name: String) -> Self {
        self.extensions.insert(name, extension);
        self
    }
    pub fn run_program(&self, program: Program) {
        eval_program(program, &self);
    }
}

impl<T: Scope + Clone> Clone for Context<T> {
    fn clone(&self) -> Self {
        Self {
            scope: self.scope.clone(),
            extensions: self.extensions.clone(),
        }
    }
}

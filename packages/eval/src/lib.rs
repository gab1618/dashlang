pub mod errors;
pub mod scope;
pub mod stdlib;
#[cfg(test)]
mod tests;

use std::{collections::HashMap, rc::Rc};

use ast::{BinaryExpr, BinaryOperator, Call, Expr, Instruction, Literal, Program, Stmt, UnaryExpr};
use errors::RuntimeResult;
use scope::Scope;

use crate::errors::RuntimeError;

macro_rules! define_aritmetic_operation {
    ($operator:tt, $op:expr, $scope:expr) => {
        match ($op.left, $op.right) {
            (Expr::Literal(left), Expr::Literal(right)) => match (left, right) {
                (Literal::Int(left), Literal::Int(right)) => Ok(Literal::Int(left $operator right)),
                (Literal::Float(left), Literal::Int(right)) => Ok(Literal::Float(left $operator (right as f64))),
                (Literal::Int(left), Literal::Float(right)) => Ok(Literal::Float((left as f64) $operator right)),
                (Literal::Float(left), Literal::Float(right)) => Ok(Literal::Float(left $operator right)),
                (_, _) => panic!("Unsuported operation"),
            },
            (left, right) => eval_binary_op(
                BinaryExpr::new(
                    Expr::Literal(eval(left, $scope)?),
                    Expr::Literal(eval(right, $scope)?),
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
                (Literal::Int(left), Literal::Int(right)) => Ok(Literal::Bool(left $operator right)),
                (Literal::Float(left), Literal::Int(right)) => Ok(Literal::Bool(left $operator (right as f64))),
                (Literal::Int(left), Literal::Float(right)) => Ok(Literal::Bool((left as f64) $operator right)),
                (Literal::Float(left), Literal::Float(right)) => Ok(Literal::Bool(left $operator right)),
                (_, _) => panic!("Unsuported operation"),
            },
            (left, right) => eval_binary_op(
                BinaryExpr::new(
                    Expr::Literal(eval(left, $scope)?),
                    Expr::Literal(eval(right, $scope)?),
                    $op.operator,
                ),
                $scope,
            ),
        }
    };
}

fn is_truthy<T: Scope + Clone>(expr: Expr, scope: &Context<T>) -> RuntimeResult<bool> {
    match expr {
        Expr::Literal(value) => match value {
            Literal::Closure(_) => Ok(true),
            Literal::Int(num) => Ok(num != 0),
            Literal::Float(num) => Ok(num != 0.0),
            Literal::String(string) => Ok(!string.is_empty()),
            Literal::Vector(val) => Ok(!val.is_empty()),
            Literal::Bool(val) => Ok(val),
            Literal::Null => Ok(false),
            Literal::Void => Ok(false),
        },
        expr => is_truthy(Expr::Literal(eval(expr, scope)?), scope),
    }
}

fn eval_binary_op<T: Scope + Clone>(op: BinaryExpr, ctx: &Context<T>) -> RuntimeResult<Literal> {
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
        BinaryOperator::And => Ok(Literal::Bool(
            is_truthy(op.left, ctx)? && is_truthy(op.right, ctx)?,
        )),
        BinaryOperator::Or => Ok(Literal::Bool(
            is_truthy(op.left, ctx)? || is_truthy(op.right, ctx)?,
        )),
    }
}
fn eval_unary_op<T: Scope + Clone>(op: UnaryExpr, ctx: &Context<T>) -> RuntimeResult<Literal> {
    match op.operator {
        ast::UnaryOperator::Not => match op.operand {
            Expr::Literal(literal) => Ok(Literal::Bool(!is_truthy(Expr::Literal(literal), ctx)?)),
            expr => {
                let literal_from_expr = eval(expr, ctx)?;
                let new_unary_op = UnaryExpr {
                    operator: ast::UnaryOperator::Not,
                    operand: Expr::Literal(literal_from_expr),
                };
                eval_unary_op(new_unary_op, ctx)
            }
        },
    }
}

pub fn eval_program<T: Scope + Clone>(
    program: Program,
    ctx: &Context<T>,
) -> RuntimeResult<Literal> {
    for instruction in program {
        match instruction {
            Instruction::Stmt(stmt) => match stmt {
                Stmt::Return(val) => {
                    return eval(val, ctx);
                }
                Stmt::If(if_stmt) => {
                    if is_truthy(if_stmt.cond, ctx)? {
                        let block_result = eval_program(if_stmt.body, ctx)?;
                        match block_result {
                            Literal::Void => (),
                            val => return Ok(val),
                        }
                    } else if let Some(else_block) = if_stmt.else_block {
                        let block_result = eval_program(else_block, ctx)?;
                        match block_result {
                            Literal::Null => (),
                            val => return Ok(val),
                        }
                    }
                }
                Stmt::While(while_stmt) => {
                    while is_truthy(while_stmt.clone().cond, ctx)? {
                        let block_result = eval_program(while_stmt.clone().body, ctx)?;
                        match block_result {
                            Literal::Void => (),
                            val => return Ok(val),
                        }
                    }
                }
                Stmt::For(for_stmt) => {
                    eval_program(vec![for_stmt.clone().init], ctx)?;
                    while is_truthy(for_stmt.clone().cond, ctx)? {
                        let block_result = eval_program(for_stmt.clone().body, ctx)?;
                        match block_result {
                            Literal::Void => (),
                            val => return Ok(val),
                        }
                        eval_program(vec![for_stmt.clone().iteration], ctx)?;
                    }
                }
            },
            Instruction::Expr(expr) => {
                eval(expr, ctx)?;
            }
        }
    }
    Ok(Literal::Void)
}

fn eval_call<T: Scope + Clone>(call: Call, ctx: &Context<T>) -> RuntimeResult<Literal> {
    if let Some(found_extension) = ctx.extensions.get(&call.symbol) {
        let local_context = ctx.clone();
        let args: Vec<Literal> = call
            .args
            .into_iter()
            .map(|expr| eval(expr, &local_context).unwrap())
            .collect();
        for (symbol, val) in found_extension.params.iter().zip(args) {
            // Inject all arguments into local scope
            local_context.scope.set(symbol, val);
        }
        return (found_extension.implementation)(&local_context);
    }
    if let Literal::Closure(closure) = ctx.scope.get(&call.symbol) {
        let local_context = ctx.clone();
        let args: Vec<Literal> = call
            .args
            .into_iter()
            .map(|expr| eval(expr, &local_context).unwrap())
            .collect();
        for (symbol, val) in closure.params.into_iter().zip(args) {
            // Inject all arguments into local scope
            local_context.scope.set(&symbol, val);
        }
        return eval_program(closure.body, &local_context);
    }
    Err(RuntimeError::new(&format!(
        "Cannot call '{}': not callable",
        call.symbol
    )))
}

pub fn eval<T: Scope + Clone>(expr: Expr, ctx: &Context<T>) -> RuntimeResult<Literal> {
    match expr {
        Expr::Literal(val) => Ok(val),
        Expr::BinaryExpr(op) => eval_binary_op(*op, ctx),
        Expr::Assignment(assign) => {
            let evaluated = eval(*assign.value, ctx)?;
            ctx.scope.set(&assign.symbol, evaluated.clone());
            Ok(evaluated)
        }
        Expr::Call(call) => eval_call(call, ctx),
        Expr::Symbol(symbol) => Ok(ctx.scope.get(&symbol)),
        Expr::UnaryExpr(op) => eval_unary_op(*op, ctx),
    }
}
type ExtensionImplementation<S> = dyn Fn(&Context<S>) -> RuntimeResult<Literal>;
#[derive(Clone)]
pub struct Extension<S: Scope> {
    pub params: Vec<String>,
    pub implementation: Rc<ExtensionImplementation<S>>,
}
pub trait Plugin<T: Scope> {
    fn get_extensions(&self) -> Vec<(String, Extension<T>)>;
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
    pub fn use_extension(&mut self, extension: Extension<T>, name: String) {
        self.extensions.insert(name, extension);
    }
    pub fn run_program(&self, program: Program) {
        if let Err(runtime_error) = eval_program(program, self) {
            eprintln!("{runtime_error}");
        }
    }
    pub fn use_plugin(&mut self, plug: &dyn Plugin<T>) {
        for (name, extension) in plug.get_extensions() {
            self.use_extension(extension, name);
        }
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

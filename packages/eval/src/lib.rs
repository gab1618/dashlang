pub mod errors;
pub mod scope;
pub mod stdlib;
#[cfg(test)]
mod tests;

use std::{cmp::Ordering, collections::HashMap, path::Path, rc::Rc};

use ast::{
    BinaryExpr, BinaryOperator, Boolean, Call, Expr, Float, Instruction, Int, Literal, Program,
    Stmt, UnaryExpr, Void,
};
use errors::RuntimeResult;
use scope::Scope;

use crate::errors::RuntimeError;

macro_rules! define_aritmetic_operation {
    ($operator:tt, $op:expr, $scope:expr, $source_path:expr) => {
        match ($op.left, $op.right) {
            (Expr::Literal(left), Expr::Literal(right)) => match (left, right) {
                (Literal::Int(left), Literal::Int(right)) => Ok(Literal::Int(Int{value: left.value $operator right.value, location: Default::default()})),
                (Literal::Float(left), Literal::Int(right)) => Ok(Literal::Float(Float{value: left.value $operator (right.value as f64), location: Default::default()})),
                (Literal::Int(left), Literal::Float(right)) => Ok(Literal::Float(Float{value: (left.value as f64) $operator right.value, location: Default::default()})),
                (Literal::Float(left), Literal::Float(right)) => Ok(Literal::Float(Float{value: left.value $operator right.value, location: Default::default()})),
                (_, _) => Err(RuntimeError::new("Unsuported operation").location($op.location)),
            },
            (left, right) => eval_binary_op(
                BinaryExpr::new(
                    Expr::Literal(eval(left, $scope, $source_path)?),
                    Expr::Literal(eval(right, $scope, $source_path)?),
                    $op.operator,
                ),
                $scope,
                $source_path
            ),
        }
    };
}

macro_rules! define_boolean_operation {
    ($operator:tt, $op:expr, $scope:expr, $source_path:expr) => {
        match ($op.left, $op.right) {
            (Expr::Literal(left), Expr::Literal(right)) => match (left, right) {
                (Literal::Int(left), Literal::Int(right)) => Ok(Literal::Bool(Boolean{value: left.value $operator right.value, location: Default::default()})),
                (Literal::Float(left), Literal::Int(right)) => Ok(Literal::Bool(Boolean{value: left.value $operator (right.value as f64), location: Default::default()})),
                (Literal::Int(left), Literal::Float(right)) => Ok(Literal::Bool(Boolean{value: (left.value as f64) $operator right.value, location: Default::default()})),
                (Literal::Float(left), Literal::Float(right)) => Ok(Literal::Bool(Boolean{value: left.value $operator right.value, location: Default::default()})),
                (_, _) => Err(RuntimeError::new("Unsuported operation").location($op.location)),
            },
            (left, right) => eval_binary_op(
                BinaryExpr::new(
                    Expr::Literal(eval(left, $scope, $source_path)?),
                    Expr::Literal(eval(right, $scope, $source_path)?),
                    $op.operator,
                ),
                $scope,
                $source_path
            ),
        }
    };
}

fn is_truthy<T: Scope + Clone, P: AsRef<Path> + Clone>(
    expr: Expr,
    scope: &Context<T, P>,
    source_path: P,
) -> RuntimeResult<bool> {
    match expr {
        Expr::Literal(value) => match value {
            Literal::Closure(_) => Ok(true),
            Literal::Int(num) => Ok(num.value != 0),
            Literal::Float(num) => Ok(num.value != 0.0),
            Literal::String(string) => Ok(!string.value.is_empty()),
            Literal::Vector(val) => Ok(!val.value.is_empty()),
            Literal::Bool(val) => Ok(val.value),
            Literal::Null(_) => Ok(false),
            Literal::Void(_) => Ok(false),
        },
        expr => is_truthy(
            Expr::Literal(eval(expr, scope, source_path.clone())?),
            scope,
            source_path,
        ),
    }
}

fn eval_binary_op<T: Scope + Clone, P: AsRef<Path> + Clone>(
    op: BinaryExpr,
    ctx: &Context<T, P>,
    source_path: P,
) -> RuntimeResult<Literal> {
    match op.operator {
        BinaryOperator::Add => define_aritmetic_operation!(+, op, ctx, source_path.clone()),
        BinaryOperator::Sub => define_aritmetic_operation!(-, op, ctx, source_path.clone()),
        BinaryOperator::Mul => define_aritmetic_operation!(*, op, ctx, source_path.clone()),
        BinaryOperator::Div => define_aritmetic_operation!(/, op, ctx, source_path.clone()),
        BinaryOperator::Gt => define_boolean_operation!(>, op, ctx, source_path.clone()),
        BinaryOperator::Eq => define_boolean_operation!(==, op, ctx, source_path.clone()),
        BinaryOperator::Ge => define_boolean_operation!(>=, op, ctx, source_path.clone()),
        BinaryOperator::Lt => define_boolean_operation!(<, op, ctx, source_path.clone()),
        BinaryOperator::Le => define_boolean_operation!(<=, op, ctx, source_path.clone()),
        BinaryOperator::And => Ok(Literal::Bool(Boolean {
            value: is_truthy(op.left, ctx, source_path.clone())?
                && is_truthy(op.right, ctx, source_path.clone())?,
            location: op.location,
        })),
        BinaryOperator::Or => Ok(Literal::Bool(Boolean {
            value: is_truthy(op.left, ctx, source_path.clone())?
                || is_truthy(op.right, ctx, source_path.clone())?,
            location: op.location,
        })),
    }
}
fn eval_unary_op<T: Scope + Clone, P: AsRef<Path> + Clone>(
    op: UnaryExpr,
    ctx: &Context<T, P>,
    source_path: P,
) -> RuntimeResult<Literal> {
    match op.operator {
        ast::UnaryOperator::Not => Ok(Literal::Bool(Boolean {
            value: !is_truthy(op.operand, ctx, source_path)?,
            location: op.location,
        })),
    }
}

pub fn eval_program<T: Scope + Clone, P: AsRef<Path> + Clone>(
    program: Program,
    ctx: &Context<T, P>,
    source_path: P,
) -> RuntimeResult<Literal> {
    for instruction in program {
        match instruction {
            Instruction::Stmt(stmt) => match stmt {
                Stmt::Return(val) => {
                    return eval(val.value, ctx, source_path.clone());
                }
                Stmt::If(if_stmt) => {
                    if is_truthy(if_stmt.cond, ctx, source_path.clone())? {
                        let block_result = eval_program(if_stmt.body, ctx, source_path.clone())?;
                        match block_result {
                            Literal::Void(_) => (),
                            val => return Ok(val),
                        }
                    } else if let Some(else_block) = if_stmt.else_block {
                        let block_result = eval_program(else_block, ctx, source_path.clone())?;
                        match block_result {
                            Literal::Null(_) => (),
                            val => return Ok(val),
                        }
                    }
                }
                Stmt::While(while_stmt) => {
                    while is_truthy(while_stmt.clone().cond, ctx, source_path.clone())? {
                        let block_result =
                            eval_program(while_stmt.clone().body, ctx, source_path.clone())?;
                        match block_result {
                            Literal::Void(_) => (),
                            val => return Ok(val),
                        }
                    }
                }
                Stmt::For(for_stmt) => {
                    eval_program(vec![for_stmt.clone().init], ctx, source_path.clone())?;
                    while is_truthy(for_stmt.clone().cond, ctx, source_path.clone())? {
                        let block_result =
                            eval_program(for_stmt.clone().body, ctx, source_path.clone())?;
                        match block_result {
                            Literal::Void(_) => (),
                            val => return Ok(val),
                        }
                        eval_program(vec![for_stmt.clone().iteration], ctx, source_path.clone())?;
                    }
                }
            },
            Instruction::Expr(expr) => {
                eval(expr, ctx, source_path.clone())?;
            }
        }
    }
    Ok(Literal::Void(Void {
        location: Default::default(),
    }))
}

fn eval_call<T: Scope + Clone, P: AsRef<Path> + Clone>(
    call: Call,
    ctx: &Context<T, P>,
    source_path: P,
) -> RuntimeResult<Literal> {
    if let Some(found_extension) = ctx.extensions.get(&call.symbol) {
        match found_extension.params.len().cmp(&call.args.len()) {
            Ordering::Less | Ordering::Greater => {
                return Err(RuntimeError::new(&format!(
                    "Could not evaluate '{}'. Expected {} arguments, but {} were given instead",
                    call.symbol,
                    found_extension.params.len(),
                    call.args.len()
                )))
            }
            Ordering::Equal => {
                let local_context = ctx.clone();
                let args: RuntimeResult<Vec<Literal>> = call
                    .args
                    .into_iter()
                    .map(|expr| eval(expr, &local_context, source_path.clone()))
                    .collect();
                match args {
                    Ok(ok_args) => {
                        for (symbol, val) in found_extension.params.iter().zip(ok_args) {
                            // Inject all arguments into local scope
                            local_context.scope.set(symbol, val);
                        }
                    }
                    Err(args_err) => return Err(args_err),
                }
                return (found_extension.implementation)(&local_context, source_path.clone());
            }
        }
    }
    if let Literal::Closure(closure) = ctx.scope.get(&call.symbol) {
        match closure.params.len().cmp(&call.args.len()) {
            Ordering::Less | Ordering::Greater => {
                return Err(RuntimeError::new(&format!(
                    "Could not evaluate '{}'. Expected {} arguments, but {} were given instead",
                    call.symbol,
                    closure.params.len(),
                    call.args.len()
                )))
            }
            Ordering::Equal => {
                let local_context = ctx.clone();
                let args: RuntimeResult<Vec<Literal>> = call
                    .args
                    .into_iter()
                    .map(|expr| eval(expr, &local_context, source_path.clone()))
                    .collect();
                match args {
                    Ok(ok_args) => {
                        for (symbol, val) in closure.params.iter().zip(ok_args) {
                            // Inject all arguments into local scope
                            local_context.scope.set(symbol, val);
                        }
                    }
                    Err(args_err) => return Err(args_err),
                }
                return eval_program(closure.body, &local_context, source_path);
            }
        }
    }
    Err(RuntimeError::new(&format!(
        "Cannot call '{}': not callable",
        call.symbol
    )))
}

pub fn eval<T: Scope + Clone, P: AsRef<Path> + Clone>(
    expr: Expr,
    ctx: &Context<T, P>,
    source_path: P,
) -> RuntimeResult<Literal> {
    match expr {
        Expr::Literal(val) => Ok(val),
        Expr::BinaryExpr(op) => eval_binary_op(*op, ctx, source_path),
        Expr::Assignment(assign) => {
            let evaluated = eval(*assign.value, ctx, source_path)?;
            ctx.scope.set(&assign.symbol, evaluated.clone());
            Ok(evaluated)
        }
        Expr::Call(call) => eval_call(call, ctx, source_path),
        Expr::Symbol(symbol) => Ok(ctx.scope.get(&symbol.value)),
        Expr::UnaryExpr(op) => eval_unary_op(*op, ctx, source_path),
    }
}
type ExtensionImplementation<S, P> = dyn Fn(&Context<S, P>, P) -> RuntimeResult<Literal>;
#[derive(Clone)]
pub struct Extension<S: Scope, P: AsRef<Path> + Clone> {
    pub params: Vec<String>,
    pub implementation: Rc<ExtensionImplementation<S, P>>,
}
pub trait Plugin<T: Scope, P: AsRef<Path> + Clone> {
    fn get_extensions(&self) -> Vec<(String, Extension<T, P>)>;
}
pub struct Context<T: Scope, P: AsRef<Path> + Clone> {
    scope: T,
    extensions: HashMap<String, Extension<T, P>>,
}
impl<T: Scope + Clone, P: AsRef<Path> + Clone> Context<T, P> {
    pub fn new(s: T) -> Self {
        Self {
            scope: s,
            extensions: HashMap::new(),
        }
    }
    pub fn use_extension(&mut self, extension: Extension<T, P>, name: String) {
        self.extensions.insert(name, extension);
    }
    pub fn run_program(&self, program: Program, source_path: P) {
        if let Err(runtime_error) = eval_program(program, self, source_path) {
            eprintln!("{runtime_error}");
        }
    }
    pub fn use_plugin(&mut self, plug: &dyn Plugin<T, P>) {
        for (name, extension) in plug.get_extensions() {
            self.use_extension(extension, name);
        }
    }
}

impl<T: Scope + Clone, P: AsRef<Path> + Clone> Clone for Context<T, P> {
    fn clone(&self) -> Self {
        Self {
            scope: self.scope.clone(),
            extensions: self.extensions.clone(),
        }
    }
}

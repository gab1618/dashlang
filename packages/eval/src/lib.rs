pub mod ctx;
pub mod extension;
pub mod scope;
pub mod stdlib;
#[cfg(test)]
mod tests;

use std::cmp::Ordering;

use ast::{
    BinaryExpr, BinaryOperator, Boolean, Call, DestructuringAsignment, Expr, Float, Int, Literal,
    Program, Stmt, Tuple, UnaryExpr, Void,
};

use ctx::Context;
use errors::{DashlangError, DashlangResult, ErrorKind, RuntimeErrorKind};
use extension::{Extension, Plugin};
use scope::Scope;

macro_rules! define_aritmetic_operation {
    ($operator:tt, $op:expr, $scope:expr) => {
        match ($op.left, $op.right) {
            (Expr::Literal(left), Expr::Literal(right)) => match (left, right) {
                (Literal::Int(left), Literal::Int(right)) => Ok(Literal::Int(Int{value: left.value $operator right.value, location: Default::default()})),
                (Literal::Float(left), Literal::Int(right)) => Ok(Literal::Float(Float{value: left.value $operator (right.value as f64), location: Default::default()})),
                (Literal::Int(left), Literal::Float(right)) => Ok(Literal::Float(Float{value: (left.value as f64) $operator right.value, location: Default::default()})),
                (Literal::Float(left), Literal::Float(right)) => Ok(Literal::Float(Float{value: left.value $operator right.value, location: Default::default()})),
                (_, _) => Err(DashlangError::new("Invalid operation", ErrorKind::Runtime(RuntimeErrorKind::InvalidOperation)).location($op.location)),
            },
            (left, right) => eval_binary_expr(
                BinaryExpr::new(
                    Expr::Literal(eval(left, $scope)?),
                    Expr::Literal(eval(right, $scope)?),
                    $op.operator,
                ),
                $scope
            ),
        }
    };
}

macro_rules! define_bitwise_operation {
    ($operator:tt, $op:expr, $scope:expr) => {
        match ($op.left, $op.right) {
            (Expr::Literal(left), Expr::Literal(right)) => match (left, right) {
                (Literal::Int(left), Literal::Int(right)) => Ok(Literal::Int(Int{value: left.value $operator right.value, location: Default::default()})),
                (_, _) => Err(DashlangError::new("Invalid operation", ErrorKind::Runtime(RuntimeErrorKind::InvalidOperation)).location($op.location)),

            }
            (left, right) => eval_binary_expr(
                BinaryExpr::new(
                    Expr::Literal(eval(left, $scope)?),
                    Expr::Literal(eval(right, $scope)?),
                    $op.operator,
                ),
                $scope
            ),
        }

    };
}

macro_rules! define_boolean_operation {
    ($operator:tt, $op:expr, $scope:expr) => {
        match ($op.left, $op.right) {
            (Expr::Literal(left), Expr::Literal(right)) => match (left, right) {
                (Literal::Int(left), Literal::Int(right)) => Ok(Literal::Bool(Boolean{value: left.value $operator right.value, location: Default::default()})),
                (Literal::Float(left), Literal::Int(right)) => Ok(Literal::Bool(Boolean{value: left.value $operator (right.value as f64), location: Default::default()})),
                (Literal::Int(left), Literal::Float(right)) => Ok(Literal::Bool(Boolean{value: (left.value as f64) $operator right.value, location: Default::default()})),
                (Literal::Float(left), Literal::Float(right)) => Ok(Literal::Bool(Boolean{value: left.value $operator right.value, location: Default::default()})),
                (_, _) => Err(DashlangError::new("Invalid operation", ErrorKind::Runtime(RuntimeErrorKind::InvalidOperation)).location($op.location)),
            },
            (left, right) => eval_binary_expr(
                BinaryExpr::new(
                    Expr::Literal(eval(left, $scope)?),
                    Expr::Literal(eval(right, $scope)?),
                    $op.operator,
                ),
                $scope
            ),
        }
    };
}

fn is_truthy<T: Scope + Clone>(expr: Expr, scope: &Context<T>) -> DashlangResult<bool> {
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
            Literal::Tuple(_) => Ok(false),
            Literal::Map(map) => Ok(!map.value.is_empty()),
        },
        expr => is_truthy(Expr::Literal(eval(expr, scope)?), scope),
    }
}

fn eval_binary_expr<T: Scope + Clone>(op: BinaryExpr, ctx: &Context<T>) -> DashlangResult<Literal> {
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
        BinaryOperator::And => {
            let left_evaluated = is_truthy(op.left, ctx)?;
            Ok(Literal::Bool(Boolean {
                value: if !left_evaluated {
                    false
                } else {
                    is_truthy(op.right, ctx)?
                },
                location: op.location,
            }))
        }
        BinaryOperator::Or => {
            let left_evaluated = is_truthy(op.left, ctx)?;
            Ok(Literal::Bool(Boolean {
                value: if left_evaluated {
                    true
                } else {
                    is_truthy(op.right, ctx)?
                },
                location: op.location,
            }))
        }
        BinaryOperator::BitwiseOr => define_bitwise_operation!(|, op, ctx),
        BinaryOperator::BitwiseAnd => define_bitwise_operation!(&, op, ctx),
        BinaryOperator::BitwiseShiftLeft => define_bitwise_operation!(<<, op, ctx),
        BinaryOperator::BitwiseShiftRight => define_bitwise_operation!(>>, op, ctx),
        BinaryOperator::BitwiseXor => define_bitwise_operation!(^, op, ctx),
    }
}
fn eval_unary_op<T: Scope + Clone>(op: UnaryExpr, ctx: &Context<T>) -> DashlangResult<Literal> {
    match op.operator {
        ast::UnaryOperator::Not => Ok(Literal::Bool(Boolean {
            value: !is_truthy(op.operand, ctx)?,
            location: op.location,
        })),
        ast::UnaryOperator::BitwiseNot => Ok(Literal::Int(Int {
            value: match eval(op.operand, ctx)? {
                Literal::Int(integer) => !integer.value,
                _ => Err(DashlangError::new(
                    "Expected integer",
                    ErrorKind::Runtime(RuntimeErrorKind::InvalidOperation),
                ))?,
            },
            location: op.location,
        })),
    }
}

pub fn eval_program<T: Scope + Clone>(
    program: Program,
    ctx: &Context<T>,
) -> DashlangResult<Literal> {
    for stmt in program {
        match stmt {
            Stmt::Return(val) => {
                return eval(val.value, ctx);
            }
            Stmt::If(if_stmt) => {
                if is_truthy(if_stmt.cond, ctx)? {
                    let block_result = eval_program(if_stmt.body, ctx)?;
                    match block_result {
                        Literal::Void(_) => (),
                        val => return Ok(val),
                    }
                } else if let Some(else_block) = if_stmt.else_block {
                    let block_result = eval_program(else_block, ctx)?;
                    match block_result {
                        Literal::Null(_) => (),
                        val => return Ok(val),
                    }
                }
            }
            Stmt::While(while_stmt) => {
                while is_truthy(while_stmt.clone().cond, ctx)? {
                    let block_result = eval_program(while_stmt.clone().body, ctx)?;
                    match block_result {
                        Literal::Void(_) => (),
                        val => return Ok(val),
                    }
                }
            }
            Stmt::For(for_stmt) => {
                eval_program(vec![for_stmt.clone().init], ctx)?;
                while is_truthy(for_stmt.clone().cond, ctx)? {
                    let block_result = eval_program(for_stmt.clone().body, ctx)?;
                    match block_result {
                        Literal::Void(_) => (),
                        val => return Ok(val),
                    }
                    eval_program(vec![for_stmt.clone().iteration], ctx)?;
                }
            }
            Stmt::Expr(expr) => {
                eval(expr, ctx)?;
            }
        }
    }
    Ok(Literal::Void(Void {
        location: Default::default(),
    }))
}

fn eval_call<T: Scope + Clone>(call: Call, ctx: &Context<T>) -> DashlangResult<Literal> {
    if let Some(found_extension) = ctx.extensions.get(&call.symbol) {
        match found_extension.params.len().cmp(&call.args.len()) {
            Ordering::Less | Ordering::Greater => {
                return Err(DashlangError::new(
                    &format!(
                        "Could not evaluate '{}'. Expected {} arguments, but {} were given instead",
                        call.symbol,
                        found_extension.params.len(),
                        call.args.len()
                    ),
                    ErrorKind::Runtime(RuntimeErrorKind::WrongArgs),
                )
                .location(call.location))
            }
            Ordering::Equal => {
                let local_context = ctx.clone();
                return (found_extension.implementation)(&local_context, call);
            }
        }
    }
    if let Literal::Closure(closure) = ctx.scope.get(&call.symbol) {
        match closure.params.len().cmp(&call.args.len()) {
            Ordering::Less | Ordering::Greater => {
                return Err(DashlangError::new(
                    &format!(
                    "Could not evaluate '{}'. Expected {} argument{s}, but {} {s1} given instead",
                    call.symbol,
                    closure.params.len(),
                    call.args.len(),
                    s = if closure.params.len() > 1_usize {"s"} else {""},
                    s1 = if call.args.len() > 1 {"were"} else {"was"}
                ),
                    ErrorKind::Runtime(RuntimeErrorKind::WrongArgs),
                )
                .location(call.location))
            }
            Ordering::Equal => {
                let local_context = ctx.clone();
                let args: Result<Vec<Literal>, DashlangError> = call
                    .args
                    .into_iter()
                    .map(|expr| eval(expr, &local_context))
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
                return eval_program(closure.body, &local_context);
            }
        }
    }
    Err(DashlangError::new(
        &format!("Cannot call '{}': not callable", call.symbol),
        ErrorKind::Runtime(RuntimeErrorKind::NonCallable),
    )
    .location(call.location))
}

fn eval_destructuring_assign_expr<T: Scope + Clone>(
    expr: DestructuringAsignment,
    ctx: &Context<T>,
) -> DashlangResult<Literal> {
    let value = eval(*expr.value, ctx)?;
    if let Literal::Tuple(tup) = value {
        let mut eval_expressions: Vec<Expr> = vec![];
        if expr.symbols.len() != tup.value.len() {
            return Err(DashlangError::new(
                "Number os elements in tuples don't match",
                ErrorKind::Runtime(RuntimeErrorKind::WrongArgs),
            )
            .location(expr.location));
        }
        for (symbol, expr) in expr.symbols.into_iter().zip(tup.value) {
            let evaluated_expr = eval(expr, ctx)?;
            eval_expressions.push(Expr::Literal(evaluated_expr.clone()));
            ctx.scope.set(&symbol.value, evaluated_expr);
        }
        Ok(Literal::Tuple(Tuple {
            value: eval_expressions,
            location: expr.location,
        }))
    } else {
        Err(DashlangError::new(
            "Expected value to be a tuple",
            ErrorKind::Runtime(RuntimeErrorKind::InvalidOperation),
        ))
    }
}

pub fn eval<T: Scope + Clone>(expr: Expr, ctx: &Context<T>) -> DashlangResult<Literal> {
    match expr {
        Expr::Literal(val) => Ok(val),
        Expr::BinaryExpr(op) => eval_binary_expr(*op, ctx),
        Expr::Assignment(assign) => {
            let evaluated = eval(*assign.value, ctx)?;
            ctx.scope.set(&assign.symbol, evaluated.clone());
            Ok(evaluated)
        }
        Expr::Call(call) => eval_call(call, ctx),
        Expr::Symbol(symbol) => Ok(ctx.scope.get(&symbol.value)),
        Expr::UnaryExpr(op) => eval_unary_op(*op, ctx),
        Expr::SubExpr(sub) => eval(*sub.value, ctx),
        Expr::DestructuringAsignment(dest) => eval_destructuring_assign_expr(dest, ctx),
    }
}

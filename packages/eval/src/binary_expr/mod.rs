use ast::{BinaryExpr, BinaryOperator, Boolean, Expr, Float, Int, Literal};
use errors::{DashlangError, DashlangResult, ErrorKind, RuntimeErrorKind};

use crate::{ctx::Context, eval, is_truthy, scope::Scope};

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

pub fn eval_binary_expr<T: Scope + Clone>(
    op: BinaryExpr,
    ctx: &Context<T>,
) -> DashlangResult<Literal> {
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

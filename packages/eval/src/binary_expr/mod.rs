use ast::{BinaryExpr, BinaryOperator, Boolean, Expr, Float, Int, Literal, Location};
use errors::{DashlangError, DashlangResult, ErrorKind, RuntimeErrorKind};
use std::ops;

use crate::{ctx::Context, eval, scope::Scope};

struct AritmeticLiteral(Literal);
impl AritmeticLiteral {
    fn get_result_location(&self, rhs: &Self) -> Location {
        Location::new(self.0.get_location().start, rhs.0.get_location().end)
    }
}
impl ops::Add for AritmeticLiteral {
    type Output = Result<Literal, DashlangError>;

    fn add(self, rhs: Self) -> Self::Output {
        let result_location = self.get_result_location(&rhs);
        match (self.0, rhs.0) {
            (Literal::Int(left), Literal::Int(right)) => Ok(Literal::Int(Int {
                value: left.value + right.value,
                location: result_location,
            })),
            (Literal::Float(left), Literal::Float(right)) => Ok(Literal::Float(Float {
                value: left.value + right.value,
                location: result_location,
            })),
            (Literal::Float(left), Literal::Int(right)) => Ok(Literal::Float(Float {
                value: left.value + right.value as f64,
                location: result_location,
            })),
            (Literal::Int(left), Literal::Float(right)) => Ok(Literal::Float(Float {
                value: left.value as f64 + right.value,
                location: result_location,
            })),
            (_, _) => Err(DashlangError::new(
                "Invalid operation",
                ErrorKind::Runtime(RuntimeErrorKind::InvalidOperation),
            )
            .location(result_location)),
        }
    }
}
impl ops::Sub for AritmeticLiteral {
    type Output = Result<Literal, DashlangError>;

    fn sub(self, rhs: Self) -> Self::Output {
        let result_location = self.get_result_location(&rhs);
        match (self.0, rhs.0) {
            (Literal::Int(left), Literal::Int(right)) => Ok(Literal::Int(Int {
                value: left.value - right.value,
                location: result_location,
            })),
            (Literal::Float(left), Literal::Float(right)) => Ok(Literal::Float(Float {
                value: left.value - right.value,
                location: result_location,
            })),
            (Literal::Float(left), Literal::Int(right)) => Ok(Literal::Float(Float {
                value: left.value - right.value as f64,
                location: result_location,
            })),

            (Literal::Int(left), Literal::Float(right)) => Ok(Literal::Float(Float {
                value: left.value as f64 - right.value,
                location: result_location,
            })),
            (_, _) => Err(DashlangError::new(
                "Invalid operation",
                ErrorKind::Runtime(RuntimeErrorKind::InvalidOperation),
            )
            .location(result_location)),
        }
    }
}

impl ops::Mul for AritmeticLiteral {
    type Output = Result<Literal, DashlangError>;

    fn mul(self, rhs: Self) -> Self::Output {
        let result_location = self.get_result_location(&rhs);
        match (self.0, rhs.0) {
            (Literal::Int(left), Literal::Int(right)) => Ok(Literal::Int(Int {
                value: left.value * right.value,
                location: result_location,
            })),
            (Literal::Float(left), Literal::Float(right)) => Ok(Literal::Float(Float {
                value: left.value * right.value,
                location: result_location,
            })),
            (Literal::Float(left), Literal::Int(right)) => Ok(Literal::Float(Float {
                value: left.value * right.value as f64,
                location: result_location,
            })),
            (Literal::Int(left), Literal::Float(right)) => Ok(Literal::Float(Float {
                value: left.value as f64 * right.value,
                location: result_location,
            })),
            (_, _) => Err(DashlangError::new(
                "Invalid operation",
                ErrorKind::Runtime(RuntimeErrorKind::InvalidOperation),
            )
            .location(result_location)),
        }
    }
}
impl ops::Div for AritmeticLiteral {
    type Output = Result<Literal, DashlangError>;

    fn div(self, rhs: Self) -> Self::Output {
        let result_location = self.get_result_location(&rhs);
        match (self.0, rhs.0) {
            (Literal::Int(left), Literal::Int(right)) => Ok(Literal::Int(Int {
                value: left.value / right.value,
                location: result_location,
            })),
            (Literal::Float(left), Literal::Float(right)) => Ok(Literal::Float(Float {
                value: left.value / right.value,
                location: result_location,
            })),
            (Literal::Float(left), Literal::Int(right)) => Ok(Literal::Float(Float {
                value: left.value / right.value as f64,
                location: result_location,
            })),
            (Literal::Int(left), Literal::Float(right)) => Ok(Literal::Float(Float {
                value: left.value as f64 / right.value,
                location: result_location,
            })),
            (_, _) => Err(DashlangError::new(
                "Invalid operation",
                ErrorKind::Runtime(RuntimeErrorKind::InvalidOperation),
            )
            .location(result_location)),
        }
    }
}

pub fn eval_binary_expr<T: Scope + Clone>(
    op: BinaryExpr,
    ctx: &Context<T>,
) -> DashlangResult<Literal> {
    let arit_lhs = AritmeticLiteral(eval(op.left, ctx)?);
    let arit_rhs = AritmeticLiteral(eval(op.right, ctx)?);
    match op.operator {
        BinaryOperator::Add => arit_lhs + arit_rhs,
        BinaryOperator::Sub => arit_lhs - arit_rhs,
        BinaryOperator::Mul => arit_lhs * arit_rhs,
        BinaryOperator::Div => arit_lhs / arit_rhs,
        BinaryOperator::Gt => todo!(),
        BinaryOperator::Eq => todo!(),
        BinaryOperator::Ge => todo!(),
        BinaryOperator::Lt => todo!(),
        BinaryOperator::Le => todo!(),
        BinaryOperator::And => todo!(),
        BinaryOperator::Or => todo!(),

        BinaryOperator::BitwiseOr => todo!(),
        BinaryOperator::BitwiseAnd => todo!(),
        BinaryOperator::BitwiseShiftLeft => todo!(),
        BinaryOperator::BitwiseShiftRight => todo!(),
        BinaryOperator::BitwiseXor => todo!(),
    }
}

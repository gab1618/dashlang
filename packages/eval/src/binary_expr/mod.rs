use ast::{BinaryExpr, BinaryOperator, Boolean, Float, Int, Literal, Location};
use errors::{DashlangError, DashlangResult, ErrorKind, RuntimeErrorKind};
use std::{cmp::Ordering, ops};

use crate::{ctx::Context, eval, is_truthy, scope::Scope};

#[derive(PartialEq)]
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
impl ops::BitOr for AritmeticLiteral {
    type Output = Result<Literal, DashlangError>;

    fn bitor(self, rhs: Self) -> Self::Output {
        let result_location = self.get_result_location(&rhs);
        match (self.0, rhs.0) {
            (Literal::Int(left), Literal::Int(right)) => Ok(Literal::Int(Int {
                value: left.value | right.value,
                location: result_location,
            })),
            (_, _) => Err(DashlangError::new(
                "Invalid operation",
                ErrorKind::Runtime(RuntimeErrorKind::InvalidOperation),
            )),
        }
    }
}
impl ops::BitAnd for AritmeticLiteral {
    type Output = Result<Literal, DashlangError>;

    fn bitand(self, rhs: Self) -> Self::Output {
        let result_location = self.get_result_location(&rhs);
        match (self.0, rhs.0) {
            (Literal::Int(left), Literal::Int(right)) => Ok(Literal::Int(Int {
                value: left.value & right.value,
                location: result_location,
            })),
            (_, _) => Err(DashlangError::new(
                "Invalid operation",
                ErrorKind::Runtime(RuntimeErrorKind::InvalidOperation),
            )),
        }
    }
}
impl ops::Shl for AritmeticLiteral {
    type Output = Result<Literal, DashlangError>;

    fn shl(self, rhs: Self) -> Self::Output {
        let result_location = self.get_result_location(&rhs);
        match (self.0, rhs.0) {
            (Literal::Int(left), Literal::Int(right)) => Ok(Literal::Int(Int {
                value: left.value << right.value,
                location: result_location,
            })),
            (_, _) => Err(DashlangError::new(
                "Invalid operation",
                ErrorKind::Runtime(RuntimeErrorKind::InvalidOperation),
            )),
        }
    }
}

impl ops::Shr for AritmeticLiteral {
    type Output = Result<Literal, DashlangError>;

    fn shr(self, rhs: Self) -> Self::Output {
        let result_location = self.get_result_location(&rhs);
        match (self.0, rhs.0) {
            (Literal::Int(left), Literal::Int(right)) => Ok(Literal::Int(Int {
                value: left.value >> right.value,
                location: result_location,
            })),
            (_, _) => Err(DashlangError::new(
                "Invalid operation",
                ErrorKind::Runtime(RuntimeErrorKind::InvalidOperation),
            )),
        }
    }
}
impl ops::BitXor for AritmeticLiteral {
    type Output = Result<Literal, DashlangError>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let result_location = self.get_result_location(&rhs);
        match (self.0, rhs.0) {
            (Literal::Int(left), Literal::Int(right)) => Ok(Literal::Int(Int {
                value: left.value ^ right.value,
                location: result_location,
            })),
            (_, _) => Err(DashlangError::new(
                "Invalid operation",
                ErrorKind::Runtime(RuntimeErrorKind::InvalidOperation),
            )),
        }
    }
}

impl PartialOrd for AritmeticLiteral {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (&self.0, &other.0) {
            (Literal::Int(lhs), Literal::Int(rhs)) => {
                if lhs.value > rhs.value {
                    Some(Ordering::Greater)
                } else if lhs.value < rhs.value {
                    return Some(Ordering::Less);
                } else {
                    Some(Ordering::Equal)
                }
            }
            (Literal::Float(lhs), Literal::Float(rhs)) => {
                if lhs.value > rhs.value {
                    Some(Ordering::Greater)
                } else if lhs.value < rhs.value {
                    return Some(Ordering::Less);
                } else {
                    Some(Ordering::Equal)
                }
            }
            (Literal::Float(lhs), Literal::Int(rhs)) => {
                if lhs.value > rhs.value as f64 {
                    Some(Ordering::Greater)
                } else if lhs.value < rhs.value as f64 {
                    return Some(Ordering::Less);
                } else {
                    Some(Ordering::Equal)
                }
            }
            (Literal::Int(lhs), Literal::Float(rhs)) => {
                if lhs.value as f64 > rhs.value {
                    Some(Ordering::Greater)
                } else if (lhs.value as f64) < rhs.value {
                    return Some(Ordering::Less);
                } else {
                    Some(Ordering::Equal)
                }
            }
            (_, _) => None,
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
        BinaryOperator::Gt => Ok(Literal::Bool(Boolean {
            value: arit_lhs > arit_rhs,
            location: op.location,
        })),
        BinaryOperator::Eq => Ok(Literal::Bool(Boolean {
            value: arit_lhs == arit_rhs,
            location: op.location,
        })),
        BinaryOperator::Ge => Ok(Literal::Bool(Boolean {
            value: arit_lhs >= arit_rhs,
            location: op.location,
        })),
        BinaryOperator::Lt => Ok(Literal::Bool(Boolean {
            value: arit_lhs < arit_rhs,
            location: op.location,
        })),
        BinaryOperator::Le => Ok(Literal::Bool(Boolean {
            value: arit_lhs <= arit_rhs,
            location: op.location,
        })),
        BinaryOperator::And => {
            if !is_truthy(ast::Expr::Literal(arit_lhs.0), ctx)? {
                return Ok(Literal::Bool(Boolean {
                    value: false,
                    location: op.location,
                }));
            }
            Ok(Literal::Bool(Boolean {
                value: is_truthy(ast::Expr::Literal(arit_rhs.0), ctx)?,
                location: op.location,
            }))
        }
        BinaryOperator::Or => {
            if is_truthy(ast::Expr::Literal(arit_lhs.0), ctx)? {
                return Ok(Literal::Bool(Boolean {
                    value: true,
                    location: op.location,
                }));
            }
            Ok(Literal::Bool(Boolean {
                value: is_truthy(ast::Expr::Literal(arit_rhs.0), ctx)?,
                location: op.location,
            }))
        }

        BinaryOperator::BitwiseOr => arit_lhs | arit_rhs,
        BinaryOperator::BitwiseAnd => arit_lhs & arit_rhs,
        BinaryOperator::BitwiseShiftLeft => arit_lhs << arit_rhs,
        BinaryOperator::BitwiseShiftRight => arit_lhs >> arit_rhs,
        BinaryOperator::BitwiseXor => arit_lhs ^ arit_rhs,
    }
}

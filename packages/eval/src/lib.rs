pub mod scope;
#[cfg(test)]
mod tests;

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

fn is_truthy<T: Scope + Clone>(expr: Expr, scope: &T) -> bool {
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

fn eval_binary_op<T: Scope + Clone>(op: BinaryExpr, scope: &T) -> Literal {
    match op.operator {
        BinaryOperator::Add => define_aritmetic_operation!(+, op, scope),
        BinaryOperator::Sub => define_aritmetic_operation!(-, op, scope),
        BinaryOperator::Mul => define_aritmetic_operation!(*, op, scope),
        BinaryOperator::Div => define_aritmetic_operation!(/, op, scope),
        BinaryOperator::Gt => define_boolean_operation!(>, op, scope),
        BinaryOperator::Eq => define_boolean_operation!(==, op, scope),
        BinaryOperator::Ge => define_boolean_operation!(>=, op, scope),
        BinaryOperator::Lt => define_boolean_operation!(<, op, scope),
        BinaryOperator::Le => define_boolean_operation!(<=, op, scope),
        BinaryOperator::And => {
            Literal::Bool(is_truthy(op.left, scope) && is_truthy(op.right, scope))
        }
        BinaryOperator::Or => {
            Literal::Bool(is_truthy(op.left, scope) || is_truthy(op.right, scope))
        }
    }
}
fn eval_unary_op<T: Scope + Clone>(op: UnaryExpr, scope: &T) -> Literal {
    match op.operator {
        ast::UnaryOperator::Not => match op.operand {
            Expr::Literal(literal) => Literal::Bool(!is_truthy(Expr::Literal(literal), scope)),
            expr => {
                let literal_from_expr = eval(expr, scope);
                let new_unary_op = UnaryExpr {
                    operator: ast::UnaryOperator::Not,
                    operand: Expr::Literal(literal_from_expr),
                };
                eval_unary_op(new_unary_op, scope)
            }
        },
    }
}

pub fn eval_program<T: Scope + Clone>(program: Program, scope: &T) -> Literal {
    for instruction in program {
        match instruction {
            Instruction::Stmt(stmt) => match stmt {
                Stmt::Return(val) => {
                    return eval(val, scope);
                }
                Stmt::If(if_stmt) => {
                    if is_truthy(if_stmt.cond, scope) {
                        let block_result = eval_program(if_stmt.body, scope);
                        match block_result {
                            Literal::Void => (),
                            val => return val,
                        }
                    } else if let Some(else_block) = if_stmt.else_block {
                        let block_result = eval_program(else_block, scope);
                        match block_result {
                            Literal::Null => (),
                            val => return val,
                        }
                    }
                }
                Stmt::While(while_stmt) => {
                    while is_truthy(while_stmt.clone().cond, scope) {
                        let block_result = eval_program(while_stmt.clone().body, scope);
                        match block_result {
                            Literal::Void => (),
                            val => return val,
                        }
                    }
                }
                Stmt::Print(expr) => {
                    let val = eval(expr, scope);
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
                eval(expr, scope);
            }
        }
    }
    Literal::Void
}

fn eval_call<T: Scope + Clone>(call: Call, scope: &T) -> Literal {
    let found = scope.get(&call.symbol);
    if let Literal::Closure(closure) = found {
        let local_scope = scope.clone();
        let args: Vec<Literal> = call
            .args
            .into_iter()
            .map(|expr| eval(expr, &local_scope))
            .collect();
        for (symbol, val) in closure.params.into_iter().zip(args) {
            // Inject all arguments into local scope
            local_scope.set(&symbol, val);
        }
        eval_program(closure.body, &local_scope)
    } else {
        panic!("Cannot call {}: not callable", call.symbol);
    }
}

pub fn eval<T: Scope + Clone>(expr: Expr, scope: &T) -> Literal {
    match expr {
        Expr::Literal(val) => val,
        Expr::BinaryExpr(op) => eval_binary_op(*op, scope),
        Expr::Asignment(asign) => {
            let evaluated = eval(*asign.value, scope);
            scope.set(&asign.symbol, evaluated.clone());
            evaluated
        }
        Expr::Call(call) => eval_call(call, scope),
        Expr::Symbol(symbol) => scope.get(&symbol),
        Expr::UnaryExpr(op) => eval_unary_op(*op, scope),
    }
}

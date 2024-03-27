use ast::Program;
use pest::Parser;

use crate::{
    instruction::parse_instruction,
    parser::{DashlangParser, Rule},
};

pub fn parse_program(input: &str) -> Program {
    let mut program: Program = vec![];
    let ast = DashlangParser::parse(Rule::program, input)
        .expect("Could not parse program")
        .next()
        .expect("Could not parse program");
    for instruction in ast.into_inner() {
        program.push(parse_instruction(instruction.as_str()));
    }
    program
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use ast::{
        Asignment, BinaryOp, BinaryOpType, Call, Closure, Expr, Instruction, Literal, Stmt, While,
    };

    use super::*;

    fn get_example_file_path<P: AsRef<Path>>(filename: P) -> PathBuf {
        let examples_dir_path = std::env::current_dir()
            .expect("Could not get current dir")
            .join(Path::new("examples"));
        examples_dir_path.join(filename)
    }
    fn get_example_program<P: AsRef<Path>>(filename: P) -> Program {
        let filepath = get_example_file_path(filename);
        let file_content = std::fs::read_to_string(&filepath).unwrap();
        let program = parse_program(&file_content);
        program
    }
    #[test]
    fn test_parse_program() {
        assert_eq!(
            parse_program("age = 5 count = 1"),
            vec![
                Instruction::Expr(Expr::Asignment(Asignment {
                    symbol: String::from("age"),
                    value: Box::new(Expr::Literal(Literal::Int(5)))
                })),
                Instruction::Expr(Expr::Asignment(Asignment {
                    symbol: String::from("count"),
                    value: Box::new(Expr::Literal(Literal::Int(1)))
                }))
            ]
        )
    }
    #[test]
    fn test_hello_world() {
        assert_eq!(
            get_example_program("hello_world.dash"),
            vec![Instruction::Stmt(Stmt::Print(Expr::Literal(
                Literal::String(String::from("Hello, World!"))
            )))]
        );
    }
    #[test]
    fn test_while() {
        assert_eq!(
            get_example_program("while.dash"),
            vec![
                Instruction::Expr(Expr::Asignment(Asignment {
                    symbol: String::from("count"),
                    value: Box::new(Expr::Literal(Literal::Int(0)))
                })),
                Instruction::Stmt(Stmt::While(While {
                    cond: Expr::BinaryOp(Box::new(BinaryOp {
                        left: Expr::Symbol(String::from("count")),
                        right: Expr::Literal(Literal::Int(10)),
                        op_type: BinaryOpType::Lt
                    })),
                    body: vec![
                        Instruction::Stmt(Stmt::Print(Expr::Symbol(String::from("count")))),
                        Instruction::Expr(Expr::Asignment(Asignment {
                            symbol: String::from("count"),
                            value: Box::new(Expr::BinaryOp(Box::new(BinaryOp {
                                left: Expr::Symbol(String::from("count")),
                                right: Expr::Literal(Literal::Int(1)),
                                op_type: BinaryOpType::Add
                            })))
                        }))
                    ]
                }))
            ]
        );
    }
    #[test]
    fn test_closure() {
        assert_eq!(
            get_example_program("say_hello.dash"),
            vec![
                Instruction::Expr(Expr::Asignment(Asignment {
                    symbol: String::from("sayHello"),
                    value: Box::new(Expr::Literal(Literal::Closure(Closure {
                        params: vec![],
                        body: vec![Instruction::Stmt(Stmt::Print(Expr::Literal(
                            Literal::String(String::from("Hello"))
                        )))]
                    })))
                })),
                Instruction::Expr(Expr::Call(Call {
                    symbol: String::from("sayHello"),
                    args: vec![]
                }))
            ]
        );
    }
    #[test]
    fn test_is_adult() {
        assert_eq!(
            get_example_program("is_adult.dash"),
            vec![Instruction::Expr(Expr::Asignment(Asignment {
                symbol: String::from("isAdult"),
                value: Box::new(Expr::Literal(Literal::Closure(Closure {
                    params: vec![String::from("age")],
                    body: vec![Instruction::Stmt(Stmt::Return(Expr::BinaryOp(Box::new(
                        BinaryOp {
                            left: Expr::Symbol(String::from("age")),
                            right: Expr::Literal(Literal::Int(18)),
                            op_type: BinaryOpType::Ge
                        }
                    ))))]
                })))
            }))]
        );
    }
}

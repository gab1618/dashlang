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
        Asignment, BinaryOp, BinaryOpType, Call, Closure, Expr, Instruction, Stmt, Value, While,
    };

    use super::*;

    fn get_example_file_path<P: AsRef<Path>>(filename: P) -> PathBuf {
        let examples_dir_path = std::env::current_dir()
            .expect("Could not get current dir")
            .join(Path::new("examples"));
        examples_dir_path.join(filename)
    }
    #[test]
    fn test_parse_program() {
        assert_eq!(
            parse_program("age = 5 count = 1"),
            vec![
                Instruction::Expr(Expr::Asignment(Asignment {
                    symbol: String::from("age"),
                    value: Box::new(Expr::Value(Value::Int(5)))
                })),
                Instruction::Expr(Expr::Asignment(Asignment {
                    symbol: String::from("count"),
                    value: Box::new(Expr::Value(Value::Int(1)))
                }))
            ]
        )
    }
    #[test]
    fn test_hello_world() {
        let hello_world_content =
            std::fs::read_to_string(get_example_file_path("hello_world.dash")).unwrap();
        let parsed = parse_program(&hello_world_content);
        assert_eq!(
            parsed,
            vec![Instruction::Stmt(Stmt::Print(Expr::Value(Value::String(
                String::from("Hello, World!")
            ))))]
        );
    }
    #[test]
    fn test_while() {
        let while_content = std::fs::read_to_string(get_example_file_path("while.dash")).unwrap();
        let parsed = parse_program(&while_content);
        assert_eq!(
            parsed,
            vec![
                Instruction::Expr(Expr::Asignment(Asignment {
                    symbol: String::from("count"),
                    value: Box::new(Expr::Value(Value::Int(0)))
                })),
                Instruction::Stmt(Stmt::While(While {
                    cond: Expr::BinaryOp(Box::new(BinaryOp {
                        left: Expr::Symbol(String::from("count")),
                        right: Expr::Value(Value::Int(10)),
                        op_type: BinaryOpType::Lt
                    })),
                    body: vec![
                        Instruction::Stmt(Stmt::Print(Expr::Symbol(String::from("count")))),
                        Instruction::Expr(Expr::Asignment(Asignment {
                            symbol: String::from("count"),
                            value: Box::new(Expr::BinaryOp(Box::new(BinaryOp {
                                left: Expr::Symbol(String::from("count")),
                                right: Expr::Value(Value::Int(1)),
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
        let say_hello_content =
            std::fs::read_to_string(get_example_file_path("say_hello.dash")).unwrap();
        let parsed = parse_program(&say_hello_content);
        assert_eq!(
            parsed,
            vec![
                Instruction::Expr(Expr::Asignment(Asignment {
                    symbol: String::from("sayHello"),
                    value: Box::new(Expr::Value(Value::Closure(Closure {
                        params: vec![],
                        body: vec![Instruction::Stmt(Stmt::Print(Expr::Value(Value::String(
                            String::from("Hello")
                        ))))]
                    })))
                })),
                Instruction::Expr(Expr::Call(Call {
                    symbol: String::from("sayHello"),
                    args: vec![]
                }))
            ]
        );
    }
}

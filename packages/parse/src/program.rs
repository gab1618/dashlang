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
    use std::path::Path;

    use ast::{Asignment, BinaryOp, BinaryOpType, Expr, Instruction, Stmt, Value, While};

    use super::*;
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
    fn test_from_file() {
        let examples_dir_path = std::env::current_dir()
            .expect("Could not get current dir")
            .join(Path::new("examples"));
        let hello_world_path = examples_dir_path.join(Path::new("hello_world.dash"));
        let hello_world_content = std::fs::read_to_string(hello_world_path).unwrap();
        let parsed = parse_program(&hello_world_content);
        assert_eq!(
            parsed,
            vec![Instruction::Stmt(Stmt::Print(Expr::Value(Value::String(
                String::from("Hello, World!")
            ))))]
        );
        let while_path = examples_dir_path.join(Path::new("while.dash"));
        let while_content = std::fs::read_to_string(while_path).unwrap();
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
}

use std::path::{Path, PathBuf};

use ast::{
    AssignmentExpr, BinaryExpr, BinaryOperator, Closure, Expr, Instruction, Int, Literal, Location,
    Program, Stmt,
};

use crate::program::parse_program;

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
fn test_is_adult() {
    assert_eq!(
        get_example_program("is_adult.dash"),
        vec![Instruction::Expr(Expr::Assignment(AssignmentExpr {
            symbol: String::from("is_adult"),
            value: Box::new(Expr::Literal(Literal::Closure(Closure {
                params: vec![String::from("age")],
                body: vec![Instruction::Stmt(Stmt::Return(Expr::BinaryExpr(Box::new(
                    BinaryExpr {
                        left: Expr::Symbol(String::from("age")),
                        right: Expr::Literal(Literal::Int(Int {
                            value: 18,
                            location: Location::new(0, 2)
                        })),
                        operator: BinaryOperator::Ge,
                        location: Location::default(),
                    }
                ))))],
                location: Location::new(0, 24),
            }))),
            location: Location::default(),
        }))]
    );
}

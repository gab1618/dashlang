use std::{env, fs::read_to_string};

use ast::Program;
use errors::{DashlangError, DashlangResult, ErrorKind, ParsingErrorKind};

use crate::parse;

fn get_example_program(program_name: &str) -> std::io::Result<String> {
    let current_dir = env::current_dir()?;
    let base_examples_folder_path = current_dir.join("./examples");
    let file_path = base_examples_folder_path.join(program_name);
    let file_content = read_to_string(file_path)?;
    Ok(file_content)
}
fn parse_example_program(program_name: &str) -> DashlangResult<Program> {
    let content = get_example_program(program_name).map_err(|err| DashlangError {
        location: None,
        message: err.to_string(),
        kind: ErrorKind::Parsing(ParsingErrorKind::Default),
    })?;
    let parsed = parse(&content)?;
    Ok(parsed)
}

#[test]
fn all_examples_compile() {
    parse_example_program("fatorial.dash").unwrap();
    parse_example_program("hello_world.dash").unwrap();
    parse_example_program("is_adult.dash").unwrap();
    parse_example_program("say_adult.dash").unwrap();
    parse_example_program("say_hello.dash").unwrap();
    parse_example_program("while.dash").unwrap();
}

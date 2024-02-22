use std::fs::read_to_string;

use eval::{eval_program, scope::HashScope};
use parse::parse;

pub fn run_file(file_path: &str) -> Result<(), std::io::Error> {
    let scope = HashScope::default();
    let file_content = read_to_string(file_path)?;
    let program = parse(&file_content);
    eval_program(program, &scope);

    Ok(())
}

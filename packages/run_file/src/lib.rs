use std::fs::read_to_string;

use eval::{scope::HashScope, stdlib::Stdlib, Context};
use parse::parse;

pub fn run_file(file_path: &str) -> Result<(), std::io::Error> {
    let scope = HashScope::default();
    let file_content = read_to_string(file_path)?;
    let mut ctx = Context::new(scope);
    ctx.use_plugin(&Stdlib {});
    let program = parse(&file_content);
    ctx.run_program(program, file_path);

    Ok(())
}

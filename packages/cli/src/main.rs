use clap::{Arg, ArgAction, Command};
use eval::{
    ctx::Context,
    scope::HashScope,
    stdlib::{stdio::Stdio, Stdlib},
};
use run_file::{error::RunfileResult, run_file};

fn main() -> RunfileResult {
    let cli = Command::new("cli")
        .arg(Arg::new("file_path").required(true).action(ArgAction::Set))
        .get_matches();
    let file_path: &String = cli
        .get_one("file_path")
        .expect("Missing file path argument");
    let scope = HashScope::default();
    let mut ctx = Context::new(scope);
    ctx.use_plugin(Stdlib::new());
    ctx.use_plugin(Stdio::new());
    run_file(file_path, &mut ctx)?;
    Ok(())
}

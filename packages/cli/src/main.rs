use clap::{Arg, ArgAction, Command};
use run_file::run_file;

fn main() -> Result<(), std::io::Error> {
    let cli = Command::new("cli")
        .arg(Arg::new("file_path").required(true).action(ArgAction::Set))
        .get_matches();
    let file_path: &String = cli
        .get_one("file_path")
        .expect("Missing file path argument");
    run_file(file_path)?;
    Ok(())
}

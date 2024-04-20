use std::{env, fs::read_dir, path::PathBuf};

use crate::{error::RunfileResult, run_file};

fn get_examples_folder_path() -> PathBuf {
    let current_dir = env::current_dir().unwrap();
    let examples_folder_path = current_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("examples");
    examples_folder_path
}

fn get_example_program_path(program_name: &str) -> PathBuf {
    let base_examples_folder_path = get_examples_folder_path();
    let file_path = base_examples_folder_path.join(program_name);
    file_path
}
fn run_example(program_name: &str) -> RunfileResult {
    run_file(get_example_program_path(program_name).to_str().unwrap())
}

fn run_all_examples(exclude: Vec<String>) -> RunfileResult {
    let examples_folder_path = get_examples_folder_path();
    let files = read_dir(examples_folder_path).unwrap();
    for file in files {
        let existing_file = file.unwrap();
        let file_name = existing_file.file_name().to_str().unwrap().to_owned();
        // Only runs if is a file and is not included in the exclude list
        if !exclude.contains(&file_name) && existing_file.path().is_file() {
            run_example(&file_name)?;
        }
    }
    Ok(())
}

#[test]
fn all_examples_run() {
    run_all_examples(vec!["greet.dash".to_owned()]).unwrap();
}

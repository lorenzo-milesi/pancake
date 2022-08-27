use std::{env, fs};

use process::include_files;

use crate::process::get_file_path;

mod process;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let (path, root) = get_file_path(&mut args);

    let content = fs::read_to_string(path).expect(&format!("Could not read file {}", path));

    fs::write("main.md", include_files(&content, root)).expect("Could not write file");
}

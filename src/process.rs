use std::{path::Path, fs};
use regex::Regex;

/*
 * Returns file path passed in argument, if nothing is passed, defaults to
 * ./main.md
 */
pub fn get_file_path(args: &mut Vec<String>) -> (&str, &str) {
    if args.len() < 2 {
        args.push("main.md".to_string());
    }

    let path = Path::new(&args[1]);

    (path.to_str().unwrap(), path.parent().unwrap().to_str().unwrap())
}

pub fn include_files(content: &String, root: &str) -> String {
    let include_regex = Regex::new(r"include::(.+)").unwrap();
    let mut new_content = content.to_string();

    for inc in include_regex.captures_iter(content) {
        // Generate
        let path = format!("{}/{}", root, &inc[1]);
        let included_content = match fs::read_to_string(&path) {
            Ok(txt) => {
                let sub_root = Path::new(&path).parent().unwrap().to_str().unwrap();
                include_files(&txt, sub_root)
            },
            Err(_)  => {
                println!("Could not read included file: {}", path);
                break;
            }
        };

        // Replace
        let from = format!("include::{}", &inc[1]);
        new_content = new_content.replace(&from, &included_content);
    }

    return new_content.to_string();
}


use std::fs;
use std::fs::File;
use std::path::Path;

pub fn cmd_touch(filename: &str) {
    match File::create(filename) {
        Ok(_) => println!("File '{}' created.", filename),
        Err(e) => println!("Failed to create file '{}': {}", filename, e),
    }
}

pub fn cmd_cat(filename: &str) {
    match fs::read_to_string(filename) {
        Ok(contents) => println!("{}", contents),
        Err(e) => println!("Error reading file '{}': {}", filename, e),
    }
}

pub fn cmd_rm(target: &str) {
    let path = Path::new(target);

    if path.is_dir() {
        match fs::remove_dir_all(path) {
            Ok(_) => println!("Directory '{}' removed.", target),
            Err(e) => println!("Error removing directory '{}': {}", target, e),
        }
    } else if path.is_file() {
        match fs::remove_file(path) {
            Ok(_) => println!("File '{}' removed.", target),
            Err(e) => println!("Error removing file '{}': {}", target, e),
        }
    } else {
        println!("No such file or directory: '{}'", target);
    }
}

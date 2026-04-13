use std::fs;

pub fn cmd_pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => println!("Error: {}", e),
    }
}

pub fn cmd_ls() {
    let entries = match fs::read_dir(".") {
        Ok(e) => e,
        Err(e) => {
            eprintln!("ls error: {}", e);
            return;
        }
    };

    for entry in entries {
        if let Ok(ent) = entry {
            let meta = ent.metadata().unwrap();
            let name = ent.file_name().into_string().unwrap();

            if meta.is_dir() {
                // Blue for folders
                println!("\x1b[34m{}\x1b[0m", name);
            } else {
                // White for files
                println!("{}", name);
            }
        }
    }
}

pub fn cmd_mkdir(name: &str) {
    match fs::create_dir(name) {
        Ok(_) => println!("Directory '{}' created.", name),
        Err(e) => println!("Failed to create directory '{}': {}", name, e),
    }
}
pub fn cmd_cd(path: &str) {
    match env::set_current_dir(path) {
        Ok(_) => (),
        Err(e) => println!("Failed to change directory to '{}': {}", path, e),
    }
}

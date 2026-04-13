use std::fs::{self, File};
use std::io::{self, Write, Read, BufReader};

pub fn open_editor(path: &str) -> Result<(), String> {
    let mut contents = String::new();

    if let Ok(file) = File::open(path) {
        let mut reader = BufReader::new(file);
        reader.read_to_string(&mut contents).map_err(|e| e.to_string())?;
    }

    println!("\n--- MINIGREP EDITOR ---");
    println!("Editing: {}", path);
    println!("Commands: :wq = save + quit, :q! = quit without saving");
    println!("-----------------------------------------\n");

    println!("{}", contents);

    let mut new_contents = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();

        match line.trim() {
            ":wq" => {
                fs::write(path, new_contents).map_err(|e| e.to_string())?;
                println!("Saved.");
                break;
            }
            ":q!" => {
                println!("Quit without saving.");
                break;
            }
            _ => new_contents.push_str(&line),
        }
    }

    Ok(())
}

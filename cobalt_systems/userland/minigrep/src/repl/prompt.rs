use users::get_current_username;
use hostname::get;

pub fn prompt() -> String {
    use std::io::{self, Write};

    let username = get_current_username()
        .map(|u| u.to_string_lossy().to_string())
        .unwrap_or("user".into());

    let hostname = get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or("host".into());

    let dir = std::env::current_dir()
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .unwrap_or("?".into());

    print!("\x1b[1;32m[{username}@{hostname} \x1b[1;34m{dir}]\x1b[0m$ ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer
}

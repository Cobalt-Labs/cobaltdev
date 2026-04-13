pub fn cmd_clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn cmd_exit() -> Result<(), String> {
    println!("Goodbye!");
    std::process::exit(0);
}

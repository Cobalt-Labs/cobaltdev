use chrono::Local;
use users::get_current_username;
use hostname::get;

pub fn cmd_clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn cmd_exit() -> Result<(), String> {
    println!("Goodbye!");
    std::process::exit(0);
}

pub fn cmd_date() {
    let now = Local::now();
    println!("{}", now.format("%a %b %e %H:%M:%S %Z %Y"));
}

pub fn cmd_whoami() {
    if let Some(username) = get_current_username() {
        println!("{}", username.to_string_lossy());
    } else {
        println!("unknown");
    }
}

pub fn cmd_uname() {
    let host = get().unwrap_or_else(|_| "unknown".into());
    let os = sys_info::os_type().unwrap_or_else(|_| "unknown".into());
    let release = sys_info::os_release().unwrap_or_else(|_| "unknown".into());
    println!("{} {} {}", os, host.to_string_lossy(), release);
}

pub fn cmd_echo(args: &[String]) {
    println!("{}", args.join(" "));
}

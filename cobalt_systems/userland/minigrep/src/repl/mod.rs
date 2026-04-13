<<<<<<< HEAD
mod prompt;
mod parser;
mod executor;
=======
pub mod prompt;
pub mod parser;
pub mod executor;
>>>>>>> 18ecb77

pub use prompt::prompt;
pub use parser::parse_input;
pub use executor::execute;

pub fn start_repl() {
    loop {
        let input = prompt();
        if let Some(cmd) = parse_input(&input) {
            if let Err(e) = execute(cmd) {
                eprintln!("Error: {e}");
            }
        }
    }
}

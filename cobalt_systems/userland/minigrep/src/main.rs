use minigrep::repl::start_repl;
use minigrep::{config::Config, run};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let config = Config::new(&args).unwrap_or_else(|err| {
            eprintln!("Argument error: {err}");
            std::process::exit(1);
        });

        if let Err(err) = run(config) {
            eprintln!("Application error: {err}");
            std::process::exit(1);
        }
    } else {
        println!("Welcome to minigrep CLI!");
        start_repl();
    }
}

pub mod config;
pub mod search;
pub mod commands;
pub mod repl;
pub mod editor;
pub mod error;

use crate::config::Config;
use crate::search::{recursive_search, search, search_case_insensitive};
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.recursive {
        recursive_search(&config.query, &config.filename, config.case_sensitive)?;
    } else {
        let contents = std::fs::read_to_string(&config.filename)?;
        let results = if config.case_sensitive {
            search(&config.query, &contents)
        } else {
            search_case_insensitive(&config.query, &contents)
        };

        for line in results {
            println!("{}", line);
        }
    }
    Ok(())
}

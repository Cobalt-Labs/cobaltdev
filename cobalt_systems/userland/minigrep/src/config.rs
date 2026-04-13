use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
    pub recursive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        match args {
            [_, flag, query, filename] if flag == "-r" => Ok(Config {
                query: query.clone(),
                filename: filename.clone(),
                case_sensitive,
                recursive: true,
            }),
            [_, query, filename] => Ok(Config {
                query: query.clone(),
                filename: filename.clone(),
                case_sensitive,
                recursive: false,
            }),
            _ => Err("Usage: minigrep [-r] <query> <filename>"),
        }
    }

}

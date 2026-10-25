use std::error::Error;
use std::fs;

pub fn recursive_search(
    query: &str,
    dir: &str,
    case_sensitive: bool,
) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            recursive_search(query, &path.to_string_lossy(), case_sensitive)?;
        } else if path.is_file() {
            let content = fs::read_to_string(&path);
            if let Ok(content) = content {
                let results = if case_sensitive {
                    search(query, &content)
                } else {
                    search_case_insensitive(query, &content)
                };
                if !results.is_empty() {
                    println!("{}:", path.display());
                    for line in results {
                        println!("{}", line);
                    }
                    println!();
                }
            }
        }
    }
    Ok(())
}

pub fn search(query: &str, contents: &str) -> Vec<String> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .map(|line| line.to_string())
        .collect()
}

pub fn search_case_insensitive(query: &str, contents: &str) -> Vec<String> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .map(|line| line.to_string())
        .collect()
}

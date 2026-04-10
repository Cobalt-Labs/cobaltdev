use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::Write;

use crate::models::models::JournalEntry;

pub fn export_to_md(entries: &[JournalEntry], file_path: &str) -> Result<()> {
    let bar = ProgressBar::new(entries.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len}")
            .unwrap()
            .progress_chars("=> "),
    );

    let mut file = File::create(file_path)?;

    for (idx, entry) in entries.iter().enumerate() {
        writeln!(file, "# {}", entry.title)?;
        writeln!(file, "_created: {}_", entry.created_at)?;
        writeln!(file, "_updated: {}_\n", entry.updated_at)?;
        writeln!(file, "{}", entry.content)?;
        
        if !entry.tags.is_empty() {
            writeln!(file, "_tags: {}_", entry.tags.join(", "))?;
        }

        writeln!(file, "\n---\n")?;

        bar.inc(1);

        if idx % 10 == 0 {
            bar.set_message(format!("Exported '{}'", entry.title));
        }
    }

    bar.finish_with_message("Export complete!");
    println!("{}", "Journal exported successfully!".green());

    Ok(())
}

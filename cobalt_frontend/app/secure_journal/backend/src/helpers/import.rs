use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;

use crate::models::models::JournalEntry;

pub fn import_md(file_path: &str) -> Result<Vec<JournalEntry>> {
    let content = fs::read_to_string(file_path)?;

    let bar = ProgressBar::new_spinner();
    bar.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    bar.set_message("Parsing markdown...");

    let mut entries = Vec::new();
    let mut current = JournalEntry::default();

    for line in content.lines() {
        let line = line.trim();

        if line.starts_with("# ") {
            if !current.title.is_empty() {
                entries.push(current);
                current = JournalEntry::default();
            }
            current.title = line.trim_start_matches("# ").to_string();
        }
        else if line.starts_with("_created:") {
            current.created_at = line
                .trim_start_matches("_created:")
                .trim()
                .trim_end_matches('_')
                .to_string();
        }
        else if line.starts_with("_updated:") {
            current.updated_at = line
                .trim_start_matches("_updated:")
                .trim()
                .trim_end_matches('_')
                .to_string();
        }
        else if line.starts_with("_tags:") {
            let tags = line
                .trim_start_matches("_tags:")
                .trim()
                .trim_end_matches('_')
                .split(',')
                .map(|t| t.trim().to_string())
                .filter(|t| !t.is_empty())
                .collect();
            current.tags = tags;
        }
        else if line == "---" {
            // end of entry
        }
        else {
            current.content.push_str(line);
            current.content.push('\n');
        }
    }

    if !current.title.is_empty() {
        entries.push(current);
    }

    bar.finish_with_message(
        format!("Imported {} entries!", entries.len()).green().to_string(),
    );

    Ok(entries)
}

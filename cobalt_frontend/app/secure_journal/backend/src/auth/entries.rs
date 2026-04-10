use anyhow::Result;
use colored::*;
use dialoguer::Input;
use chrono::Utc;
use tokio::time::{sleep, Duration};
use sqlx::Row;

use crate::models::models::{JournalEntry, User};
use crate::db::DbPool;

pub async fn new_entry(db: &DbPool, user: &User) -> Result<()> {
    let title = Input::<String>::new()
        .with_prompt("Title")
        .interact()?;

    let content = Input::<String>::new()
        .with_prompt("Content")
        .interact()?;

    let tags_input = Input::<String>::new()
        .with_prompt("Tags (comma separated)")
        .allow_empty(true)
        .interact()?;

    let tags: Vec<String> = tags_input
        .split(',')
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .collect();

    let now = Utc::now().to_rfc3339();
    let tags_json = serde_json::to_string(&tags)?;

    sqlx::query(
        "INSERT INTO entries (user_id, title, content, tags, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
    )
    .bind(user.id)
    .bind(&title)
    .bind(&content)
    .bind(&tags_json)
    .bind(&now)
    .bind(&now)
    .execute(db)
    .await?;

    sleep(Duration::from_millis(500)).await;
    println!("{}", "Journal entry has been saved!".green());

    Ok(())
}

pub async fn delete_entry(db: &DbPool, user: &User) -> Result<()> {
    let entries = get_entries_for_user(db, user).await?;

    if entries.is_empty() {
        println!("{}", "No entries to delete.".red());
        return Ok(());
    }

    println!("Your journal entries:");
    for (i, entry) in entries.iter().enumerate() {
        println!("{}. {} - {}", i + 1, entry.title, entry.content);
    }

    let index = Input::<usize>::new()
        .with_prompt("Enter the number of the entry to delete")
        .interact()?;

    if index == 0 || index > entries.len() {
        println!("{}", "Invalid entry number.".red());
        return Ok(());
    }

    sqlx::query("DELETE FROM entries WHERE id = ?1")
        .bind(entries[index - 1].id.unwrap())
        .execute(db)
        .await?;

    println!("{}", "Journal entry deleted successfully!".green());
    Ok(())
}

pub async fn list_users(db: &DbPool) -> Result<()> {
    let rows = sqlx::query("SELECT username FROM users")
        .fetch_all(db)
        .await?;

    println!("{}", "Registered users:".bright_green());
    for row in rows {
        let username: String = row.get("username");
        println!("- {}", username);
    }

    Ok(())
}

pub async fn list_entries(db: &DbPool, user: &User) -> Result<()> {
    let rows = sqlx::query(
        "SELECT id, title, content, tags, created_at, updated_at
         FROM entries WHERE user_id = ?1"
    )
    .bind(user.id)
    .fetch_all(db)
    .await?;

    if rows.is_empty() {
        println!("{}", format!("No entries found for {}", user.username).red());
        return Ok(());
    }

    println!("Your journal entries:");

    for (i, row) in rows.iter().enumerate() {
        let tags: Vec<String> =
            serde_json::from_str(&row.get::<String, _>("tags")).unwrap_or_default();

        println!(
            "\n{}. {}\n   Content: {}\n   Created: {}\n   Tags: {}\n",
            i + 1,
            row.get::<String, _>("title"),
            row.get::<String, _>("content"),
            row.get::<String, _>("created_at"),
            if tags.is_empty() { "(none)".to_string() } else { tags.join(", ") }
        );
    }

    Ok(())
}

pub async fn update_entry(db: &DbPool, user: &User) -> Result<()> {
    let entries = get_entries_for_user(db, user).await?;

    if entries.is_empty() {
        println!("{}", "No entries to update.".red());
        return Ok(());
    }

    println!("Your journal entries:");
    for (i, entry) in entries.iter().enumerate() {
        println!("{}. {} - {}", i + 1, entry.title, entry.content);
    }

    let index = Input::<usize>::new()
        .with_prompt("Enter entry number to update")
        .interact()?;

    if index == 0 || index > entries.len() {
        println!("{}", "Invalid entry number.".red());
        return Ok(());
    }

    let entry = &entries[index - 1];

    let new_content = Input::<String>::new()
        .with_prompt("New content")
        .interact()?;

    let new_tags = Input::<String>::new()
        .with_prompt("New tags (comma separated)")
        .allow_empty(true)
        .interact()?;

    let tags: Vec<String> = new_tags
        .split(',')
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .collect();

    let updated_at = Utc::now().to_rfc3339();
    let tags_json = serde_json::to_string(&tags)?;

    sqlx::query(
        "UPDATE entries
         SET content = ?1, tags = ?2, updated_at = ?3
         WHERE id = ?4"
    )
    .bind(&new_content)
    .bind(&tags_json)
    .bind(&updated_at)
    .bind(entry.id.unwrap())
    .execute(db)
    .await?;

    println!("{}", "Entry updated successfully.".bright_green());
    Ok(())
}

pub async fn get_entries_for_user(db: &DbPool, user: &User) -> Result<Vec<JournalEntry>> {
    let rows = sqlx::query(
        "SELECT id, title, content, tags, created_at, updated_at
         FROM entries WHERE user_id = ?1"
    )
    .bind(user.id)
    .fetch_all(db)
    .await?;

    let mut entries = Vec::new();

    for row in rows {
        entries.push(JournalEntry {
            id: Some(row.get::<i64, _>("id")),
            user_id: user.username.clone(),
            title: row.get("title"),
            content: row.get("content"),
            tags: serde_json::from_str(&row.get::<String, _>("tags")).unwrap_or_default(),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        });
    }

    Ok(entries)
}

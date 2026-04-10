use anyhow::Result;
use colored::*;
use dialoguer::Confirm;

use crate::models::models::User;
use crate::db::DbPool;

pub async fn delete_user(db: &DbPool, user: &User) -> Result<()> {
    let confirm = Confirm::new()
        .with_prompt(format!(
            "Are you sure you want to delete '{}' and all their entries?",
            user.username
        ))
        .default(false)
        .interact()?;

    if !confirm {
        println!("{}", "Deletion cancelled.".yellow());
        return Ok(());
    }

    sqlx::query("DELETE FROM entries WHERE user_id = ?1")
        .bind(user.id)
        .execute(db)
        .await?;

    sqlx::query("DELETE FROM users WHERE id = ?1")
        .bind(user.id)
        .execute(db)
        .await?;

    println!("{}", "User and all their entries deleted successfully!".green());

    Ok(())
}

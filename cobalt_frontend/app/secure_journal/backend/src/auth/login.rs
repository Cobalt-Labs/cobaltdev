use sqlx::Row;
use colored::*;
use anyhow::Result;
use crate::db::DbPool;
use serde::Deserialize;
use indicatif::ProgressBar;
use rpassword::read_password;
use crate::models::models::User;
use argon2::{Argon2, PasswordHash, PasswordVerifier};

#[derive(Deserialize)]
pub struct _AuthRequest {
    pub username: String,
    pub password: String,
}

pub async fn login_flow(db: &DbPool) -> Result<Option<User>> {
    let mut username = String::new();
    print!("{} ", "Enter username:".blue());
    std::io::Write::flush(&mut std::io::stdout())?;
    std::io::stdin().read_line(&mut username)?;
    let username = username.trim().to_string();

    print!("{} ", "Enter password:".blue());
    std::io::Write::flush(&mut std::io::stdout())?;
    let password = read_password()?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Checking credentials...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(120));

    let row = sqlx::query(
        "SELECT id, username, password_hash FROM users WHERE username = ?1"
    )
    .bind(&username)
    .fetch_optional(db)
    .await?;

    spinner.finish_and_clear();

    let Some(row) = row else {
        println!("{}", "No such user.".red());
        return Ok(None);
    };

    let stored_hash: String = row.get("password_hash");

    let parsed_hash = PasswordHash::new(&stored_hash)?;
    if Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok() {
        println!("{}", "Login successful!".green());

        let user = User {
            id: row.get("id"),
            username: row.get("username"),
            password_hash: stored_hash,
        };

        Ok(Some(user))
    } else {
        println!("{}", "Wrong password.".red());
        Ok(None)
    }
}

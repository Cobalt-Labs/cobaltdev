use anyhow::{Result, bail};
use colored::*;

pub fn validate_creds(username: &str, password: &str) -> Result<()> {
    let username = username.trim();

    if username.is_empty() {
        bail!("{}", "Username cannot be empty.".yellow());
    }

    if !(3..=20).contains(&username.len()) {
        bail!(
            "{}",
            "Username must be between 3 and 20 characters.".yellow()
        );
    }

    if username.contains(' ') {
        bail!("{}", "Username cannot contain spaces.".yellow());
    }

    //
    if password.len() < 6 {
        bail!(
            "{}",
            "Password must be at least 6 characters long.".yellow()
        );
    }

    if !password.chars().any(|c| c.is_uppercase()) {
        bail!(
            "{}",
            "Password must contain at least one uppercase letter.".yellow()
        );
    }

    if !password.chars().any(|c| c.is_lowercase()) {
        bail!(
            "{}",
            "Password must contain at least one lowercase letter.".yellow()
        );
    }

    if !password.chars().any(|c| c.is_ascii_digit()) {
        bail!("{}", "Password must contain at least one number.".yellow());
    }

    const SPECIALS: &str = "!@#$%^&*()-_=+[]{};:,.<>?";

    if !password.chars().any(|c| SPECIALS.contains(c)) {
        bail!(
            "{}",
            "Password must contain at least one special character.".yellow()
        );
    }

    Ok(())
}

use colored::Colorize;
use dialoguer::Select;
use sqlx::{Pool, Sqlite};

use crate::auth::delete::delete_user;
use crate::auth::entries::{
    delete_entry, get_entries_for_user, list_entries, list_users, new_entry, update_entry,
};
use crate::auth::login::login_flow;
use crate::auth::signup::signup_flow;

use crate::helpers::export::export_to_md;
use crate::helpers::import::import_md;
use crate::models::models::User;

pub async fn main_menu(db: &Pool<Sqlite>) {
    let mut curr_user: Option<User> = None;

    loop {
        let options = vec![
            "Login",
            "Create account",
            "List users",
            "Write new journal entry",
            "View my entries",
            "Update an entry",
            "Delete an entry",
            "Delete my account",
            "Export journal",
            "Import journal",
            "Logout",
            "Exit",
        ];

        let choice = Select::new()
            .with_prompt("What would you like to do?")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        let result = match choice {
            0 => match login_flow(db).await {
                Ok(Some(user)) => {
                    println!("Logged in as {}", user.username.green());
                    curr_user = Some(user);
                    Ok(())
                }
                Ok(None) => {
                    println!("{}", "Login failed".red());
                    Ok(())
                }
                Err(e) => Err(e),
            },
            1 => signup_flow(db).await,
            2 => list_users(db).await,
            3 => match &curr_user {
                Some(user) => new_entry(db, user).await,
                None => {
                    println!("{}", "Please login first".red());
                    Ok(())
                }
            },
            4 => match &curr_user {
                Some(user) => list_entries(db, user).await,
                None => {
                    println!("{}", "Please login first".red());
                    Ok(())
                }
            },
            5 => match &curr_user {
                Some(user) => update_entry(db, user).await,
                None => {
                    println!("{}", "Please login first".red());
                    Ok(())
                }
            },
            6 => match &curr_user {
                Some(user) => delete_entry(db, user).await,
                None => {
                    println!("{}", "Please login first".red());
                    Ok(())
                }
            },
            7 => {
                if let Some(user) = &curr_user {
                    let _ = delete_user(db, user).await;
                    curr_user = None;
                } else {
                    println!("{}", "Please login first".red());
                }
                Ok(())
            }
            8 => {
                if let Some(user) = &curr_user {
                    let entries_res = get_entries_for_user(&db, user).await;

                    match entries_res {
                        Ok(entries) => {
                            let file_name = format!(
                                "journal_export_{}_{}.md",
                                user.username,
                                chrono::Local::now().format("%Y-%m-%d_%H-%M")
                            );

                            if let Err(e) = export_to_md(&entries, &file_name) {
                                eprintln!("{}", format!("Export failed: {:?}", e).bright_red());
                            } else {
                                println!("{}", format!("Exported to {}", file_name).green());
                            }
                        }
                        Err(e) => {
                            eprintln!("{}", format!("Failed to fetch entries: {:?}", e).red());
                        }
                    }
                } else {
                    println!("{}", "Please login first".red());
                }
                Ok(())
            }
            9 => {
                println!("{}", "Enter path to .md file:".cyan());
                let mut path = String::new();
                std::io::stdin().read_line(&mut path).unwrap();
                let path = path.trim();

                match import_md(path) {
                    Ok(entries) => {
                        for e in entries {
                            println!("{}", format!("Imported: {}", e.title).green());
                        }
                    }
                    Err(e) => eprintln!("{}", format!("Import failed: {}", e).red()),
                }

                Ok(())
            }
            10 => {
                curr_user = None;
                println!("{}", "Logged out".yellow());
                Ok(())
            }
            11 => {
                println!("{}", "Goodbye!".cyan());
                return;
            }

            _ => Ok(()),
        };

        if let Err(err) = result {
            eprintln!("{}", format!("Error: {:?}", err).bright_red());
        }
    }
}

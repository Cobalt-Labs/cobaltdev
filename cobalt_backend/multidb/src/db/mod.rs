pub mod mysql;
pub mod sqlite;

pub use anyhow::Result;
pub use mysql::MySqlDatabase;
pub use sqlite::SqliteDatabase;

use crate::models::user::User;

#[derive(Clone)]
pub enum DatabaseType {
    Sqlite,
    MySql,
    None, 
}

#[derive(Clone)]
pub struct DatabaseManager {
    pub sqlite: Option<SqliteDatabase>,
    pub mysql: Option<MySqlDatabase>,
}
impl DatabaseManager {
    pub async fn new() -> Result<Self> {
        let sqlite_url = std::env::var("DATABASE_URL_SQLITE")
            .unwrap_or_else(|_| "sqlite://./cobalt_multidb.sqlite?mode=rwc".to_string());

        let mysql_url = std::env::var("DATABASE_URL_MYSQL")
            .unwrap_or_else(|_| "mysql://root:password@localhost:3306/cobalt_multidb".to_string());

        let mut sqlite = None;
        let mut mysql = None;

        match SqliteDatabase::new(&sqlite_url).await {
            Ok(db) => {
                println!("Sqlite Connected!");
                sqlite = Some(db);
            }
            Err(e) => eprintln!("Connected to sqlite failed: {e:?}"),
        }
        match MySqlDatabase::new(&mysql_url).await {
            Ok(db) => {
                println!("MySql Connected!");
                mysql = Some(db);
            }
            Err(e) => eprintln!("Connected to mysql failed: {e:?}"),
        }

        Ok(Self { sqlite, mysql })
    }

    pub async fn create_user(&self, name: &str, db_type: DatabaseType) -> Result<Vec<User>> {
        let mut users = Vec::new();

        match db_type {
            DatabaseType::Sqlite => {
                if let Some(db) = &self.sqlite {
                    let user = db.create_user(name).await?;
                    users.push(user);
                }
            }
            DatabaseType::MySql => {
                if let Some(db) = &self.mysql {
                    let user = db.create_user(name).await?;
                    users.push(user);
                }
            }
            DatabaseType::None => eprintln!("No Databases found/selected")
        }
        Ok(users)
    }

    pub async fn get_users(&self, db_type: DatabaseType) -> Result<Vec<User>> {
        match db_type {
            DatabaseType::Sqlite => {
                if let Some(db) = &self.sqlite {
                    return Ok(db.get_user().await?);
                }
            }
            DatabaseType::MySql => {
                if let Some(db) = &self.mysql {
                    return Ok(db.get_user().await?);
                }
            }
            DatabaseType::None => eprintln!("No Databases found/selected")
        }
        Ok(vec![])
    }
}

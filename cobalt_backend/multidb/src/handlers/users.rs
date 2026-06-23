use axum::{
    extract::{State, Query},
    Json,
    http::StatusCode,
};
use serde::Deserialize;
use serde_json::json;
use crate::models::user::CreateUserReq;
use crate::db::{DatabaseManager, DatabaseType};

#[derive(Deserialize)]
pub struct DbQuery {
    pub db: Option<String>,
}

pub async fn create_user(
    State(db): State<DatabaseManager>,
    Json(payload): Json<CreateUserReq>,
) -> (StatusCode, Json<serde_json::Value>) {
    if payload.name.trim().is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "message": "Name is required"
            })),
        );
    }

    let mut results = Vec::new();
    let db_types = [
        (DatabaseType::Sqlite, "sqlite"),
        (DatabaseType::MySql, "mysql"),
    ];

    for (db_type, db_name) in db_types {
        match db.create_user(payload.name.trim(), db_type).await {
            Ok(users) => {
                for user in users {
                    results.push(json!({
                        "database": db_name,
                        "user": {
                            "id": user.id,
                            "name": user.name,
                            "created_at": user.created_at,
                        }
                    }));
                }
            }
            Err(e) => {
                results.push(json!({
                    "database": db_name,
                    "error": e.to_string(),
                }));
            }
        }
    }

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "message": format!("User '{}' created in all databases", payload.name.trim()),
            "results": results,
        })),
    )
}

pub async fn get_users(
    State(db): State<DatabaseManager>,
    Query(params): Query<DbQuery>,
) -> (StatusCode, Json<serde_json::Value>) {
    let db_name = params.db.as_deref().unwrap_or("sqlite");

    let db_type = match db_name {
        "sqlite" => DatabaseType::Sqlite,
        "mysql" => DatabaseType::MySql,
        other => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "success": false,
                    "error": format!("Unknown database '{}'. Use 'sqlite' or 'mysql'.", other)
                })),
            );
        }
    };

    match db.get_users(db_type).await {
        Ok(users) => {
            let count = users.len();
            (StatusCode::OK, Json(json!({
                "success": true,
                "database": db_name,
                "users": users,
                "count": count,
            })))
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": e.to_string(),
            })),
        ),
    }
}

pub async fn get_all_users(
    State(db): State<DatabaseManager>,
) -> (StatusCode, Json<serde_json::Value>) {
    let mut all_results = Vec::new();
    let db_types = [
        (DatabaseType::Sqlite, "sqlite"),
        (DatabaseType::MySql, "mysql"),
    ];

    for (db_type, db_name) in db_types {
        match db.get_users(db_type).await {
            Ok(users) => {
                let count = users.len();
                all_results.push(json!({
                    "database": db_name,
                    "users": users,
                    "count": count,
                }));
            }
            Err(e) => {
                all_results.push(json!({
                    "database": db_name,
                    "error": e.to_string(),
                }));
            }
        }
    }

    (StatusCode::OK, Json(json!({
        "success": true,
        "databases": all_results,
    })))
}
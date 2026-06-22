use axum::{
    extract::{State, Query},
    Json,
    http::StatusCode,
};
use serde_json::json;
use crate::models::user::{CreateUserReq, UserResp};
use crate::db::{DatabaseManager, DatabaseType};

pub async fn create_user(
    State(db): State<DatabaseManager>,
    Json(payload): Json<CreateUserReq>,
) -> (StatusCode, Json<serde_json::Value>) {
    if payload.name.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "message": "Name is required"
            })),
        );
    }

    // Create in all three databases
    let mut results = Vec::new();
    let db_types = [
        (DatabaseType::Sqlite, "sqlite"),
        (DatabaseType::MySql, "mysql"),
    ];

    for (db_type, name) in db_types {
        match db.create_user(&payload.name, db_type).await {
            Ok(users) => {
                for user in users {
                    results.push(json!({
                        "database": name,
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
                    "database": name,
                    "error": e.to_string(),
                }));
            }
        }
    }

    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "message": format!("User '{}' created in all databases", payload.name),
            "results": results,
        })),
    )
}

pub async fn get_users(
    State(db): State<DatabaseManager>,
    Query(params): Query<serde_json::Value>,
) -> (StatusCode, Json<serde_json::Value>) {
    let db_type = params
        .get("db")
        .and_then(|v| v.as_str())
        .unwrap_or("sqlite");

    let db_type_enum = match db_type {
        "mysql" => DatabaseType::MySql,
        "surreal" => DatabaseType::Sqlite,
        _ => DatabaseType::None,
    };

    match db.get_users(db_type_enum).await {
        Ok(users) => {
            (StatusCode::OK, Json(json!({
                "success": true,
                "database": db_type,
                "users": users,
                "count": users.len(),
            })))
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
                "success": false,
                "error": e.to_string(),
            })))
        }
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

    for (db_type, name) in db_types {
        match db.get_users(db_type).await {
            Ok(users) => {
                all_results.push(json!({
                    "database": name,
                    "users": users,
                    "count": users.len(),
                }));
            }
            Err(e) => {
                all_results.push(json!({
                    "database": name,
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
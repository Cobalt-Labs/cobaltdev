use axum::{Json, extract::{Path, State}};
use sqlx::{SqlitePool, Row};
use crate::models::{Todo, CreateTodo};

pub async fn create_todo(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateTodo>,
) -> Json<Todo> {
    let result = sqlx::query(
        "INSERT INTO todos (title) VALUES (?)"
    )
    .bind(&payload.title)
    .execute(&pool)
    .await
    .unwrap();

    let id = result.last_insert_rowid();

    Json(Todo {
        id,
        title: payload.title,
        completed: false,
    })
}

pub async fn get_todos(
    State(pool): State<SqlitePool>,
) -> Json<Vec<Todo>> {

    let rows = sqlx::query(
        "SELECT id, title, completed FROM todos"
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    let todos = rows.into_iter().map(|row| {
        Todo {
            id: row.get("id"),
            title: row.get("title"),
            completed: row.get("completed"),
        }
    }).collect();

    Json(todos)
}

pub async fn delete_todo(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) {
    sqlx::query(
        "DELETE FROM todos WHERE id = ?"
    )
    .bind(id)
    .execute(&pool)
    .await
    .unwrap();
}

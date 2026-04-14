use sqlx::SqlitePool;
use crate::models::task::{Task, CreateTask};

pub async fn create_task(pool: &SqlitePool, payload: CreateTask) -> Result<Task, String> {
    let result = sqlx::query(
        "INSERT INTO tasks (title, description, completed, user_id) VALUES (?, ?, false, ?)"
    )
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(payload.user_id)
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create task: {}", e))?;

    let id = result.last_insert_rowid();

    Ok(Task {
        id,
        title: payload.title,
        description: payload.description,
        completed: false,
        user_id: payload.user_id,
    })
}

pub async fn get_all_tasks(pool: &SqlitePool) -> Result<Vec<Task>, String> {
    let tasks = sqlx::query_as("SELECT id, title, description, completed, user_id FROM tasks")
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to fetch tasks: {}", e))?;

    Ok(tasks)
}

pub async fn delete_task(pool: &SqlitePool, id: i64) -> Result<(), String> {
    sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete task: {}", e))?;

    Ok(())
}
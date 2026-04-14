//expand later 
use axum::{extract::State, Json, http::StatusCode, extract::Path};
use crate::state::AppState;
use crate::models::task::{Task, CreateTask};

pub async fn create_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateTask>,
) -> Result<Json<Task>, (StatusCode, String)> {
    // TODO: Add authentication check later
    let task = crate::services::task_service::create_task(&state.db, payload).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(task))
}

pub async fn get_tasks(
    State(state): State<AppState>,
) -> Result<Json<Vec<Task>>, (StatusCode, String)> {
    let tasks = crate::services::task_service::get_all_tasks(&state.db).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(tasks))
}

pub async fn delete_task(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    crate::services::task_service::delete_task(&state.db, id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(serde_json::json!({"status": "deleted"})))
}
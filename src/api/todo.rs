use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Serialize};
use crate::error::AppResult;

#[derive(Serialize)]
pub struct Todo {
    id: i32,
    title: String,
    description: Option<String>,
    completed: bool,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

pub async fn get_todos() -> AppResult<Json<Vec<Todo>>> {
    Ok(Json(vec![]))
}
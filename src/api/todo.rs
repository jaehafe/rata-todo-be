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
    let todos = Todo {
        id: 1,
        title: "Sample Todo".to_string(),
        description: Some("This is a sample todo item.".to_string()),
        completed: false,
        created_at: Utc::now(),
        updated_at: None,
    };
    Ok(Json(vec![todos]))
}
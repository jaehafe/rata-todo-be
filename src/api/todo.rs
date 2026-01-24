use axum::{Extension, Json};
use axum::extract::Path;
use crate::db::DbPool;
use crate::error::AppResult;
use crate::models::{Todo, NewTodo};

pub async fn get_todos(
    Extension(pool): Extension<DbPool>,
) -> AppResult<Json<Vec<Todo>>> {
    let todos = Todo::list(&pool).await?;
    Ok(Json(todos))
}

pub async fn get_todo(
    Extension(pool): Extension<DbPool>,
    Path(id): Path<i32>,
) -> AppResult<Json<Todo>> {
    let todo = Todo::find(&pool, id).await?;
    Ok(Json(todo))
}

pub async fn create_todo(
    Extension(pool): Extension<DbPool>,
    Json(new_todo): Json<NewTodo>,
) -> AppResult<Json<Todo>> {
    let todo = Todo::create(&pool, new_todo).await?;
    Ok(Json(todo))
}

pub async fn delete_todo(
    Extension(pool): Extension<DbPool>,
    Path(id): Path<i32>,
) -> AppResult<Json<usize>> {
    let deleted = Todo::delete(&pool, id).await?;
    Ok(Json(deleted))
}

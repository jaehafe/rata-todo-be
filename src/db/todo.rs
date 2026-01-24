use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use crate::db::DbPool;
use crate::error::AppResult;
use crate::models::{Todo, NewTodo};
use crate::schema::todo;

impl Todo {
    pub async fn list(pool: &DbPool) -> AppResult<Vec<Todo>> {
        let mut conn = pool.get().await?;
        let todos = todo::table
            .select(Todo::as_select())
            .load(&mut conn)
            .await?;
        Ok(todos)
    }

    pub async fn find(pool: &DbPool, todo_id: i32) -> AppResult<Todo> {
        let mut conn = pool.get().await?;
        let result = todo::table
            .find(todo_id)
            .select(Todo::as_select())
            .first(&mut conn)
            .await?;
        Ok(result)
    }

    pub async fn create(pool: &DbPool, new_todo: NewTodo) -> AppResult<Todo> {
        let mut conn = pool.get().await?;
        let result = diesel::insert_into(todo::table)
            .values(&new_todo)
            .returning(Todo::as_returning())
            .get_result(&mut conn)
            .await?;
        Ok(result)
    }

    pub async fn update_completed(pool: &DbPool, todo_id: i32, completed: bool) -> AppResult<Todo> {
        let mut conn = pool.get().await?;
        let result = diesel::update(todo::table.find(todo_id))
            .set(todo::completed.eq(completed))
            .returning(Todo::as_returning())
            .get_result(&mut conn)
            .await?;
        Ok(result)
    }

    pub async fn delete(pool: &DbPool, todo_id: i32) -> AppResult<usize> {
        let mut conn = pool.get().await?;
        let result = diesel::delete(todo::table.find(todo_id))
            .execute(&mut conn)
            .await?;
        Ok(result)
    }
}

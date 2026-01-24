use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::todo;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = todo)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = todo)]
pub struct NewTodo {
    pub title: String,
    pub description: Option<String>,
}

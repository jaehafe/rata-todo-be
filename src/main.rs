mod api;
mod db;
mod error;
mod models;
mod schema;

use std::env;
use axum::{Extension, Router};
use axum::routing::{get, post, delete};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::build_db_pool(&database_url);

    tracing::info!("Starting server");

    let routes = Router::new()
        .route("/api/v1/todos", get(api::todo::get_todos))
        .route("/api/v1/todos", post(api::todo::create_todo))
        .route("/api/v1/todos/{id}", get(api::todo::get_todo))
        .route("/api/v1/todos/{id}", delete(api::todo::delete_todo));

    let app = Router::new()
        .merge(routes)
        .layer(Extension(pool));

    let addr = format!("0.0.0.0:{}", 8080);
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

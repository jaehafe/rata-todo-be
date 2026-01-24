mod api;
mod error;

use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    tracing::info!("🚀 Starting server":);

    let routes = Router::new()
        .route("/api/v1/get_todos", get(api::todo::get_todos));


    let app = Router::new()
        .merge(routes);
        // .layer(cors)
        // .layer(io_layer);

    let addr = format!("0.0.0.0:{}", 8080);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

use std::env;

use axum::routing::{get, Router};
use log::info;

mod controllers;
mod database;
mod errors;
mod models;
mod router;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_dir().unwrap().parent().unwrap().join(".env");
    dotenv::from_path(path).ok();
    env_logger::init();

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let addr = format!("{}:{}", host, port);

    let db = database::init().await.unwrap();

    info!("Starting server on {}", addr);
    let app = Router::new()
        .route("/", get(controllers::health))
        .nest("/api/v1", router::create_router())
        .with_state(db);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

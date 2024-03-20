use axum::http;
use log::info;

pub mod auth_controller;
pub mod user_controller;

pub async fn health() -> (http::StatusCode, &'static str) {
    info!("Health checked");
    (http::StatusCode::OK, "OK")
}

use axum::http;
use log::info;

pub mod auth_controller;

pub async fn health() -> http::StatusCode {
    info!("Health checked");
    http::StatusCode::OK
}

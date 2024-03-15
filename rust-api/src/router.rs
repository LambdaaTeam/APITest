use axum::{routing::post, Router};
use mongodb::Database;

use crate::controllers;

pub fn create_router() -> Router<Database> {
    Router::new()
        .route("/register", post(controllers::auth_controller::register))
        .route("/login", post(controllers::auth_controller::login))
}

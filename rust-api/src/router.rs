use axum::{routing::get, Router};
use mongodb::Database;

use crate::controllers;

pub fn create_router() -> Router<Database> {
    Router::new()
        .route("/register", get(controllers::auth_controller::register))
        .route("/login", get(controllers::auth_controller::login))
}

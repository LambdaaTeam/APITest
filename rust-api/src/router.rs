use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use mongodb::Database;

use crate::{controllers, middlewares::jwt_auth};

pub fn create_router(db: Database) -> Router<Database> {
    Router::new()
        .route("/register", post(controllers::auth_controller::register))
        .route("/login", post(controllers::auth_controller::login))
        .route(
            "/me",
            get(controllers::user_controller::me)
                .route_layer(middleware::from_fn_with_state(db, jwt_auth::auth)),
        )
}

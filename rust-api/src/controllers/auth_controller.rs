use axum::{extract, http};
use mongodb::Database;

use crate::{
    errors::ApiErrorResponse,
    models::user_model::{CreateUser, LoginUser, PublicUser},
    services,
};

pub async fn register(
    extract::State(db): extract::State<Database>,
    axum::Json(user): axum::Json<CreateUser>,
) -> Result<
    (http::StatusCode, axum::Json<PublicUser>),
    (http::StatusCode, axum::Json<ApiErrorResponse>),
> {
    let res = services::user_service::create(&db, user).await?;
    Ok(res)
}

pub async fn login(
    extract::State(db): extract::State<Database>,
    axum::Json(user): axum::Json<LoginUser>,
) -> Result<
    (http::StatusCode, axum::Json<PublicUser>),
    (http::StatusCode, axum::Json<ApiErrorResponse>),
> {
    let res = services::user_service::login(&db, user).await?;
    Ok(res)
}

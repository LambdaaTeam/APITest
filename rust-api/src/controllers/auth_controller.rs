use axum::{extract::State, http::StatusCode, Json};
use mongodb::Database;

use crate::{
    errors::ApiErrorResponse,
    models::user_model::{CreateUser, LoginUser, PublicUser, PublicUserWithToken},
    services,
};

pub async fn register(
    State(db): State<Database>,
    Json(user): Json<CreateUser>,
) -> Result<(StatusCode, Json<PublicUser>), (StatusCode, Json<ApiErrorResponse>)> {
    let res = services::user_service::create(&db, user).await?;
    Ok(res)
}

pub async fn login(
    State(db): State<Database>,
    Json(login_user): Json<LoginUser>,
) -> Result<(StatusCode, Json<PublicUserWithToken>), (StatusCode, Json<ApiErrorResponse>)> {
    let res = services::user_service::login(&db, login_user).await?;
    Ok(res)
}

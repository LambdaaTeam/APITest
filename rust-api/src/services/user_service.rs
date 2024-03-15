use axum::{http::StatusCode, Json};
use mongodb::{bson::doc, Database};

use crate::{
    errors::{ApiError, ApiErrorResponse},
    models::user_model::{CreateUser, LoginUser, PublicUser, User},
};

use super::password_service;

pub async fn create(
    db: &Database,
    user: CreateUser,
) -> Result<(StatusCode, Json<PublicUser>), (StatusCode, Json<ApiErrorResponse>)> {
    let collection = db.collection::<User>("users");
    let user = User::create(user);

    let result = collection.insert_one(user.clone(), None).await;

    match result {
        Ok(_) => Ok((StatusCode::CREATED, Json(PublicUser::from(user)))),
        Err(_) => Err(ApiError::InternalServerError.get_response()),
    }
}

pub async fn login(
    db: &Database,
    login_user: LoginUser,
) -> Result<(StatusCode, Json<PublicUser>), (StatusCode, Json<ApiErrorResponse>)> {
    let collection = db.collection::<User>("users");

    let result = collection
        .find_one(doc! { "email": login_user.email}, None)
        .await;

    match result {
        Ok(Some(user)) => {
            let hash = user.password.bytes.clone();

            if password_service::verify_password(&hash, &login_user.password) {
                Ok((StatusCode::OK, Json(PublicUser::from(user))))
            } else {
                Err(ApiError::PasswordNotMatch.get_response())
            }
        }
        Ok(None) => Err(ApiError::UserNotFound.get_response()),
        Err(_) => Err(ApiError::InternalServerError.get_response()),
    }
}

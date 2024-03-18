use axum::{http::StatusCode, Extension, Json};

use crate::{
    errors::ApiErrorResponse,
    models::user_model::{PublicUser, User},
};

pub async fn me(
    Extension(user): Extension<User>,
) -> Result<Json<PublicUser>, (StatusCode, Json<ApiErrorResponse>)> {
    Ok(Json(PublicUser::from(user)))
}

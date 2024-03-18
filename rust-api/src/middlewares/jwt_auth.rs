use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
    Json,
};
use jsonwebtoken::DecodingKey;
use mongodb::{bson::doc, Database};

use crate::{
    errors::{ApiError, ApiErrorResponse},
    models::{jwt::Claims, user_model::User},
};

pub async fn auth(
    State(db): State<Database>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiErrorResponse>)> {
    let token = req
        .headers()
        .get("Authorization")
        .ok_or(ApiError::InvalidToken.get_response())?
        .to_str()
        .map_err(|_| ApiError::InvalidToken.get_response())?;

    let token = token.split(' ').collect::<Vec<&str>>();

    if token.len() != 2 {
        return Err(ApiError::InvalidToken.get_response());
    }

    let token = token[1];

    let secret = std::env::var("JWT_SECRET").unwrap_or("secret".to_string());
    let claims = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )
    .map_err(|_| ApiError::InvalidToken.get_response())?
    .claims;

    let user = db
        .collection::<User>("users")
        .find_one(doc! { "_id": &claims.sub }, None)
        .await
        .map_err(|_| ApiError::InvalidToken.get_response())?;

    if user.is_none() {
        return Err(ApiError::InvalidToken.get_response());
    }

    req.extensions_mut().insert(user.unwrap());

    Ok(next.run(req).await)
}

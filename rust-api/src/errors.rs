use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiErrorResponse {
    pub message: String,
    pub code: u16,
}

/**
 * ApiError enum
 * 1000 - 1999: User errors
 * 2000 - 2999: Auth errors
 */

#[allow(dead_code)]
pub enum ApiError {
    InternalServerError,
    CreateUserError,
    PasswordNotMatch,
    UserNotFound,
    InvalidToken,
}

impl ApiError {
    pub fn get_response(&self) -> (StatusCode, Json<ApiErrorResponse>) {
        match self {
            // User errors
            ApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiErrorResponse {
                    message: "Internal server error".to_string(),
                    code: 0,
                }),
            ),
            ApiError::CreateUserError => (
                StatusCode::BAD_REQUEST,
                Json(ApiErrorResponse {
                    message: "Error creating user".to_string(),
                    code: 1001,
                }),
            ),
            ApiError::PasswordNotMatch => (
                StatusCode::UNAUTHORIZED,
                Json(ApiErrorResponse {
                    message: "Password not match".to_string(),
                    code: 1002,
                }),
            ),
            ApiError::UserNotFound => (
                StatusCode::NOT_FOUND,
                Json(ApiErrorResponse {
                    message: "User not found".to_string(),
                    code: 1003,
                }),
            ),
            // Auth errors
            ApiError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                Json(ApiErrorResponse {
                    message: "Invalid token".to_string(),
                    code: 2001,
                }),
            ),
        }
    }
}

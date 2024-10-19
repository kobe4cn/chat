use axum::{
    body::Body,
    response::{IntoResponse, Response},
    Json,
};

use serde::{Deserialize, Serialize};

use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorOutput {
    pub error: String,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("email already exists: {0}")]
    EmailAlreadyExists(String),
    #[error("workspace not exists: {0}")]
    WorkSpaceNotExists(String),
    #[error("sqlx error {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("password hash eoor {0}")]
    PasswordHashError(#[from] argon2::password_hash::Error),
    #[error("jwt error {0}")]
    JwtError(#[from] jwt_simple::Error),
    #[error("http header error {0}")]
    HttpHeaderError(#[from] axum::http::header::InvalidHeaderValue),
}

impl ErrorOutput {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        let status_code = match &self {
            AppError::SqlxError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::PasswordHashError(_) => axum::http::StatusCode::UNPROCESSABLE_ENTITY,
            AppError::JwtError(_) => axum::http::StatusCode::FORBIDDEN,
            AppError::HttpHeaderError(_) => axum::http::StatusCode::UNAUTHORIZED,
            AppError::EmailAlreadyExists(_) => axum::http::StatusCode::CONFLICT,
            AppError::WorkSpaceNotExists(_) => axum::http::StatusCode::NOT_FOUND,
        };
        (status_code, Json(ErrorOutput::new(self.to_string()))).into_response()
    }
}

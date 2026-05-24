use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("project not found")]
    NotFound,
    #[error("request validation failed")]
    Validation(#[from] ValidationErrors),
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    details: Option<serde_json::Value>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    error: self.to_string(),
                    details: None,
                },
            ),
            Self::Validation(errors) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    error: "request validation failed".to_owned(),
                    details: Some(serde_json::json!(errors)),
                },
            ),
        };

        (status, Json(body)).into_response()
    }
}

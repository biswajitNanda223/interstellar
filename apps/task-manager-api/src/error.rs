use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// ### RUST LEARN: Idiomatic Error Handling via Enums
/// Unlike many languages that use exceptions (which can be thrown anywhere and are hard to track),
/// Rust handles errors using the `Result<T, E>` type where errors are explicit.
/// Here we define an `AppError` enum representing all possible errors in our application.
#[derive(Debug)]
pub enum AppError {
    /// The requested task ID was not found.
    TaskNotFound(u64),
    /// Failed to acquire a lock on our database (concurrency error).
    DatabaseLockError,
}

/// ### RUST LEARN: Traits (IntoResponse)
/// Axum uses the `IntoResponse` trait to convert any custom type into an HTTP response.
/// By implementing this trait for our `AppError` enum, we can easily return `Err(AppError)`
/// from any endpoint, and Axum will automatically format it into the correct HTTP status and JSON response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Pattern matching on the enum variant.
        // The compiler guarantees that we handle all cases!
        let (status, error_message) = match self {
            AppError::TaskNotFound(id) => (
                StatusCode::NOT_FOUND,
                format!("Task with ID {} not found", id),
            ),
            AppError::DatabaseLockError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Internal concurrency error: failed to lock database"),
            ),
        };

        // Create a JSON payload with the error details.
        let body = Json(json!({
            "status": "error",
            "message": error_message
        }));

        // Return a tuple of (Status Code, JSON body) which itself implements IntoResponse!
        (status, body).into_response()
    }
}

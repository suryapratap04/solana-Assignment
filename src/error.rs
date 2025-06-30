use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid Input: {0}")]
    InvalidInput(String),

    #[error("Internal Server Error")]
    InternalError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error_message = self.to_string();
        let body = Json(json!({ "success": false, "error": error_message }));

        (StatusCode::OK, body).into_response()
    }
}

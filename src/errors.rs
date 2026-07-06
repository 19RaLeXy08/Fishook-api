use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

pub enum AppError {
    NotFound(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(what) => (StatusCode::NOT_FOUND, format!("{what} not found")),
        };
        (status, Json(json!({ "error": message}))).into_response()
    }
}
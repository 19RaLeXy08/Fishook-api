use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, Json};

use crate::errors::AppError;
use crate::models::CapturedRequest;
use crate::state::AppState;

pub async fn list_requests(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Vec<CapturedRequest>>, AppError> {
    let Some(bucket) = state.buckets.get(&id) else {
        return Err(AppError::NotFound("bucket".into()));
    };
    
    Ok(Json(bucket.requests.clone()))
}

pub async fn clear_requests(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    let Some(mut bucket) = state.buckets.get_mut(&id) else {
        return Err(AppError::NotFound("bucket".into()));
    };
    bucket.requests.clear();
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_request(
    State(state): State<Arc<AppState>>,
    Path((id, req_id)): Path<(String, String)>,
) -> Result<Json<CapturedRequest>, AppError> {
    let Some(bucket) = state.buckets.get(&id) else {
        return Err(AppError::NotFound("bucket".into()));
    };

    let Some(req) = bucket.requests.iter().find(|r| r.id == req_id) else {
        return Err(AppError::NotFound("request".into()));
    };
    Ok(Json(req.clone()))
}

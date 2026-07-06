use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, Json};
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::BucketSummary;
use crate::state::{AppState, Bucket};

pub async fn create_bucket(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let id = Uuid::new_v4().to_string();
    state.buckets.insert(id.clone(), Bucket::new());
    Json(serde_json::json!({
        "id": id,
        "hook_url": format!("http://localhost:3000/hook/{id}"),
    }))
}

pub async fn list_buckets(State(state): State<Arc<AppState>>) -> Json<Vec<BucketSummary>>{
    let buckets = state
        .buckets
        .iter()
        .map(|e| BucketSummary {
            id: e.key().clone(),
            request_count: e.requests.len(),
            created_at: e.value().created_at,
        })
        .collect();
    Json(buckets)
}

pub async fn get_bucket(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<BucketSummary>, AppError> {
    let Some(bucket) = state.buckets.get(&id) else {
        return Err(AppError::NotFound("bucket".into()));
    };
    Ok(Json(BucketSummary {
        id: id.clone(),
        request_count: bucket.requests.len(),
        created_at: bucket.created_at,
    }))
}

pub async fn delete_bucket(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    match state.buckets.remove(&id) {
        Some(_) => Ok(StatusCode::NO_CONTENT),
        None => Err(AppError::NotFound("bucket".into())),
    }
}
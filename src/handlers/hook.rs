use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::{HeaderMap, Method, StatusCode, Uri};
use chrono::Utc;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::CapturedRequest;
use crate::state::AppState;


pub async fn capture(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: String,
) -> Result<StatusCode, AppError> {
    let Some(mut bucket) = state.buckets.get_mut(&id) else {
        return Err(AppError::NotFound("bucket".into()));
    };

    let headers_map: HashMap<String, String> = headers
        .iter()
        .map(|(k, v)| (k.to_string(), String::from_utf8_lossy(v.as_bytes()).to_string()))
        .collect();

        let req = CapturedRequest {
            id: Uuid::new_v4().to_string(),
            method: method.to_string(),
            path: uri.to_string(),
            headers: headers_map,
            body,
            received_at: Utc::now(),
        };

        bucket.requests.push(req.clone());
        let _ = bucket.tx.send(req);

        Ok(StatusCode::OK)
}
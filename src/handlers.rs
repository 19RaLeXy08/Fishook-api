use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::{HeaderMap, Method, StatusCode, Uri},
    Json,
};
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::{IntoResponse, Response};
use tokio::sync::broadcast;
use chrono::Utc;
use uuid::Uuid;

use crate::models::CapturedRequest;
use crate::state::{AppState, Bucket};

pub async fn create_bucket(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let id = Uuid::new_v4().to_string();
    state.buckets.insert(id.clone(), Bucket::new());
    Json(serde_json::json!({
        "id": id,
        "hook_url": format!("http://localhost:3000/hook/{id}"),
    }))
}

pub async fn capture(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: String,
) -> StatusCode {
    let Some(mut bucket) = state.buckets.get_mut(&id) else {
        return StatusCode::NOT_FOUND;
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

        StatusCode::OK
}

pub async fn list_requests(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Vec<CapturedRequest>>, StatusCode> {
    let Some(bucket) = state.buckets.get(&id) else {
        return Err(StatusCode::NOT_FOUND);
    };
    
    Ok(Json(bucket.requests.clone()))
}

pub async fn stream(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    ws: WebSocketUpgrade,
) -> Response {
    let Some(bucket) = state.buckets.get(&id) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let rx = bucket.tx.subscribe();
    drop(bucket); 

    ws.on_upgrade(move |socket| handle_socket(socket, rx))
}

async fn handle_socket(mut socket: WebSocket, mut rx: broadcast::Receiver<CapturedRequest>) {
    while let Ok(req) = rx.recv().await {
        let Ok(json) = serde_json::to_string(&req) else {
            continue;
        };
        if socket.send(Message::Text(json.into())).await.is_err() {
            break;
        }
    }
} 



    

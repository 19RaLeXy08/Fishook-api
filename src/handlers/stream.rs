use std::sync::Arc;

use axum::extract::{Path, State};
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::{IntoResponse, Response};
use tokio::sync::broadcast;

use crate::errors::AppError;
use crate::models::CapturedRequest;
use crate::state::AppState;

pub async fn stream(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    ws: WebSocketUpgrade,
) -> Response {
    let Some(bucket) = state.buckets.get(&id) else {
        return AppError::NotFound("bucket".into()).into_response();
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
mod models;
mod state;
mod handlers;
mod errors;

use std::sync::Arc;
use axum::{routing::{get, post, any}, Router};
use state::AppState;
use handlers::{create_bucket, capture, list_requests, stream, list_buckets, get_bucket, delete_bucket, clear_requests};

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::default());

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/buckets", post(create_bucket).get(list_buckets))
        .route("/buckets/{id}", get(get_bucket).delete(delete_bucket))
        .route("/hook/{id}", any(capture))
        .route("/buckets/{id}/requests", get(list_requests).delete(clear_requests))
        .route("/buckets/{id}/stream", get(stream))
        .with_state(state);


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
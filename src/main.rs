mod models;
mod state;
mod handlers;

use std::sync::Arc;
use axum::{routing::{get, post, any}, Router};
use state::AppState;
use handlers::{create_bucket, capture, list_requests};

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::default());

    let app = Router::new()
        .route("/", get(|| async { "Fichhook alive"}))
        .route("/bucket", post(create_bucket))
        .route("/hook/{id}", any(capture))
        .route("/bucket/{id}", get(list_requests))
        .with_state(state);


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
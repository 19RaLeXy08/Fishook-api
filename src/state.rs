use dashmap::DashMap;
use tokio::sync::broadcast;
use crate::models::CapturedRequest;
use chrono::{DateTime, Utc};

pub struct Bucket {
    pub requests: Vec<CapturedRequest>,
    pub tx: broadcast::Sender<CapturedRequest>,
    pub created_at: DateTime<Utc>,
}

impl Bucket {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(64);
        Self {
            requests: Vec::new(),
            tx,
            created_at: Utc::now(),
        }
    }
}

#[derive(Default)]
pub struct AppState {
    pub buckets: DashMap<String, Bucket>,
}
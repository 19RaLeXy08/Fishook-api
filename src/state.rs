use dashmap::DashMap;
use tokio::sync::broadcast;
use crate::models::CapturedRequest;

pub struct Bucket {
    pub requests: Vec<CapturedRequest>,
    pub tx: broadcast::Sender<CapturedRequest>,
}

impl Bucket {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(64);
        Self {
            requests: Vec::new(),
            tx,
        }
    }
}

#[derive(Default)]
pub struct AppState {
    pub buckets: DashMap<String, Bucket>,
}
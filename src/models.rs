use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct CapturedRequest {
    pub id: String,
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub received_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BucketSummary {
    pub id: String,
    pub request_count: usize,
    pub created_at: DateTime<Utc>,
}
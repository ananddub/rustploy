use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
pub struct HealthReport {
    pub id: Option<i64>,
    pub target_id: i64,
    pub target_type: String,
    pub status: String,
    pub response_time_ms: i32,
    pub error_message: Option<String>,
    pub created_at: i64,
}

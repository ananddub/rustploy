use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserPolicy {
    pub id: Option<i64>,
    pub user_id: i64,
    pub org_id: i64,
    pub policy_id: i64,
    pub effect: String,
    pub created_at: i64,
}

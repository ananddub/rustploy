use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::FromRow)]
pub struct ResourceAccess {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub org_id: Option<i64>,
    pub resource_type: Option<String>,
    pub resource_id: Option<i64>,
    pub created_at: Option<i64>,
}

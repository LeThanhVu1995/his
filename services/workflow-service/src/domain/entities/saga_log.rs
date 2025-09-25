use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct SagaLog {
    pub id: Uuid,
    pub instance_id: Uuid,
    pub step_id: String,
    pub action: String,
    pub request: Option<serde_json::Value>,
    pub response: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

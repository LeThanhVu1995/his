use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Instance {
    pub id: Uuid,
    pub template_code: String,
    pub template_version: Option<i32>,
    pub status: String,
    pub input: Option<serde_json::Value>,
    pub context: serde_json::Value,
    pub cursor: serde_json::Value,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub next_wake_at: Option<DateTime<Utc>>,
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Task {
    pub id: Uuid,
    pub instance_id: Uuid,
    pub step_id: String,
    pub name: String,
    pub assignee: Option<Uuid>,
    pub candidate_roles: Option<Vec<String>>,
    pub payload: Option<serde_json::Value>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

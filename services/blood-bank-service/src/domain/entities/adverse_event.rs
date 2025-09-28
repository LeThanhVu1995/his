use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct BloodAdverseEvent {
    pub event_id: Uuid,
    pub issue_id: Uuid,
    pub event_time: DateTime<Utc>,
    pub type_code: Option<String>,
    pub severity_code: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

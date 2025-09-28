use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Issue {
    pub issue_id: Uuid,
    pub unit_id: Uuid,
    pub encounter_id: Uuid,
    pub issued_at: DateTime<Utc>,
    pub issued_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

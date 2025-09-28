use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Crossmatch {
    pub crossmatch_id: Uuid,
    pub patient_id: Uuid,
    pub unit_id: Uuid,
    pub performed_at: DateTime<Utc>,
    pub result_code: String,
    pub performer_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

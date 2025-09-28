use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct BloodRequest {
    pub request_id: Uuid,
    pub patient_id: Uuid,
    pub encounter_id: Option<Uuid>,
    pub ordering_provider: Option<Uuid>,
    pub blood_group: String,
    pub component_code: String,
    pub quantity: i32,
    pub priority: String,
    pub indication: Option<String>,
    pub status: String,
    pub requested_by: Option<Uuid>,
    pub requested_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

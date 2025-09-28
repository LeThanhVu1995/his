use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct BloodDonation {
    pub donation_id: Uuid,
    pub donor_id: Uuid,
    pub collected_at: DateTime<Utc>,
    pub volume_ml: Option<i32>,
    pub remarks: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

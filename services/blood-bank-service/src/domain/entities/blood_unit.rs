use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct BloodUnit {
    pub unit_id: Uuid,
    pub donation_id: Uuid,
    pub component_code: Option<String>, // WB, PRBC, FFP, PLT
    pub unit_no: Option<String>,
    pub blood_group: Option<String>,
    pub expiry_date: Option<NaiveDate>,
    pub status: String, // AVAILABLE, ISSUED, EXPIRED, etc.
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ClinicalOrder {
    pub order_id: String,
    pub encounter_id: String,
    pub patient_id: String,
    pub order_type: String,
    pub status: String,
    pub ordered_by: Option<String>,
    pub ordered_at: DateTime<Utc>,
    pub priority_code: Option<String>,
    pub remarks: Option<String>,
}

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
// use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VitalSignRecord {
    pub vs_id: String,
    pub encounter_id: String,
    pub patient_id: String,
    pub measured_at: DateTime<Utc>,
    pub recorder_staff_id: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VitalSignItem {
    pub vs_item_id: String,
    pub vs_id: String,
    pub code: String,
    pub value_num: Option<f64>,
    pub value_text: Option<String>,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Observation {
    pub obs_id: String,
    pub encounter_id: String,
    pub patient_id: String,
    pub code: String,
    pub value_num: Option<f64>,
    pub value_text: Option<String>,
    pub unit: Option<String>,
    pub taken_at: DateTime<Utc>,
    pub performer_staff_id: Option<String>,
    pub status: String,
}

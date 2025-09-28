use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AllergyIntolerance {
    pub allergy_id: String,
    pub patient_id: String,
    pub substance_code: String,
    pub reaction_text: Option<String>,
    pub severity_code: Option<String>,
    pub status: String,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MedicationStatement {
    pub med_stmt_id: String,
    pub patient_id: String,
    pub drug_code: String,
    pub drug_name: String,
    pub dose_text: Option<String>,
    pub frequency_text: Option<String>,
    pub route_code: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub status: String,
}

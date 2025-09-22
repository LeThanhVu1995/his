use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct LabTest {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub specimen_type: String,
    pub unit: Option<String>,
    pub ref_low: Option<f64>,
    pub ref_high: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Specimen {
    pub id: Uuid,
    pub specimen_no: String,
    pub order_id: Option<Uuid>,
    pub patient_id: Uuid,
    pub encounter_id: Option<Uuid>,
    pub specimen_type: String,
    pub collected_at: Option<DateTime<Utc>>,
    pub collected_by: Option<String>,
    pub status: String,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct LabResult {
    pub id: Uuid,
    pub result_no: String,
    pub specimen_id: Uuid,
    pub test_id: Uuid,
    pub status: String,
    pub verified_by: Option<String>,
    pub verified_at: Option<DateTime<Utc>>,
    pub released_by: Option<String>,
    pub released_at: Option<DateTime<Utc>>,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct ResultValue {
    pub id: Uuid,
    pub result_id: Uuid,
    pub analyte_code: String,
    pub analyte_name: String,
    pub value_num: Option<f64>,
    pub value_text: Option<String>,
    pub unit: Option<String>,
    pub ref_low: Option<f64>,
    pub ref_high: Option<f64>,
    pub flag: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

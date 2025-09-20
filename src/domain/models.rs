use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Medication {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub strength: Option<String>,
    pub form: Option<String>,
    pub route: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Prescription {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub encounter_id: Option<Uuid>,
    pub presc_no: String,
    pub status: String,
    pub ordered_by: Option<String>,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct PrescriptionItem {
    pub id: Uuid,
    pub prescription_id: Uuid,
    pub medication_id: Uuid,
    pub dose: Option<String>,
    pub freq: Option<String>,
    pub duration: Option<String>,
    pub qty: Decimal,
    pub instruction: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Dispense {
    pub id: Uuid,
    pub prescription_id: Uuid,
    pub disp_no: String,
    pub dispensed_by: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

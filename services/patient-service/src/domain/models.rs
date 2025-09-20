use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct Patient {
    pub id: Uuid,
    pub mrn: Option<String>,
    pub national_id: Option<String>,
    pub passport_no: Option<String>,
    pub full_name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: String,
    pub birth_date: Option<NaiveDate>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub blood_type: Option<String>,
    pub marital_status: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct Encounter {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub encounter_no: String,
    pub encounter_type: String,
    pub status: String,
    pub department_code: Option<String>,
    pub attending_doctor_id: Option<String>,
    pub admitted_at: Option<DateTime<Utc>>,
    pub discharged_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

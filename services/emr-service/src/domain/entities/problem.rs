use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Patient {
    pub patient_id: String,
    pub hospital_id: String,
    pub code: Option<String>,
    pub national_id: Option<String>,
    pub full_name: String,
    pub date_of_birth: Option<NaiveDate>,
    pub gender: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub district: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PatientIdentifier {
    pub patient_identifier_id: String,
    pub patient_id: String,
    pub system_code: String,
    pub value: String,
    pub active: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PatientContact {
    pub patient_contact_id: String,
    pub patient_id: String,
    pub relation_code: Option<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub is_primary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct EpisodeOfCare {
    pub episode_id: String,
    pub patient_id: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub status: String,
    pub reason_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Encounter {
    pub encounter_id: String,
    pub patient_id: String,
    pub episode_id: Option<String>,
    pub facility_id: String,
    pub department_id: Option<String>,
    pub room_id: Option<String>,
    pub bed_id: Option<String>,
    pub type_code: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub status: String,
    pub attending_staff_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ClinicalNote {
    pub note_id: String,
    pub encounter_id: String,
    pub author_staff_id: Option<String>,
    pub category_code: Option<String>,
    pub content_text: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProblemList {
    pub problem_id: String,
    pub patient_id: String,
    pub code: Option<String>,
    pub description: Option<String>,
    pub onset_date: Option<NaiveDate>,
    pub abatement_date: Option<NaiveDate>,
    pub status: String,
}

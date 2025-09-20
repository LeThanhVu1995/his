use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct CreatePatientReq {
    #[validate(length(min = 1, message = "Full name is required"))]
    pub full_name: String,
    #[validate(length(min = 1, message = "Gender is required"))]
    pub gender: String,
    pub birth_date: Option<chrono::NaiveDate>,
    pub mrn: Option<String>,
    pub national_id: Option<String>,
    pub passport_no: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct UpdatePatientReq {
    #[validate(length(min = 1, message = "Full name cannot be empty"))]
    pub full_name: Option<String>,
    #[validate(length(min = 1, message = "Gender cannot be empty"))]
    pub gender: Option<String>,
    pub birth_date: Option<chrono::NaiveDate>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PatientRes {
    pub id: Uuid,
    pub mrn: Option<String>,
    pub full_name: String,
    pub gender: String,
    pub birth_date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct PatientQuery {
    pub q: Option<String>,     // search by name/mrn
    #[validate(range(min = 1, message = "Page must be at least 1"))]
    pub page: Option<i64>,
    #[validate(range(min = 1, max = 200, message = "Page size must be between 1 and 200"))]
    pub page_size: Option<i64>,
}

pub fn calc_etag(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("\"{:x}\"", hasher.finalize())
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDate;

// Create Donor
#[derive(Debug, Deserialize)]
pub struct CreateDonorRequest {
    pub code: Option<String>,
    pub name: String,
    pub date_of_birth: Option<NaiveDate>,
    pub gender: Option<String>,
    pub blood_group: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateDonorResponse {
    pub donor_id: Uuid,
    pub message: String,
}

// List Donors
#[derive(Debug, Deserialize)]
pub struct ListDonorsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub blood_group: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListDonorsResponse {
    pub donors: Vec<crate::domain::entities::donor::BloodDonor>,
    pub total: usize,
    pub limit: i64,
    pub offset: i64,
}

// Get Donor
#[derive(Debug, Serialize)]
pub struct GetDonorResponse {
    pub donor: crate::domain::entities::donor::BloodDonor,
}

// Update Donor
#[derive(Debug, Deserialize)]
pub struct UpdateDonorRequest {
    pub name: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub gender: Option<String>,
    pub blood_group: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateDonorResponse {
    pub message: String,
}

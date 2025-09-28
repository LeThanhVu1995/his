use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Create Donation
#[derive(Debug, Deserialize)]
pub struct CreateDonationRequest {
    pub donor_id: Uuid,
    pub volume_ml: Option<i32>,
    pub remarks: Option<String>,
    pub component_codes: Vec<String>, // e.g., ["WB", "PRBC", "FFP"]
    pub blood_group: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateDonationResponse {
    pub donation_id: Uuid,
    pub unit_ids: Vec<Uuid>,
    pub message: String,
}

// List Donations
#[derive(Debug, Deserialize)]
pub struct ListDonationsQuery {
    pub donor_id: Option<Uuid>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub from_date: Option<chrono::NaiveDate>,
    pub to_date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Serialize)]
pub struct ListDonationsResponse {
    pub donations: Vec<crate::domain::entities::donation::BloodDonation>,
    pub total: usize,
    pub limit: i64,
    pub offset: i64,
}

// Get Donation
#[derive(Debug, Serialize)]
pub struct GetDonationResponse {
    pub donation: crate::domain::entities::donation::BloodDonation,
    pub units: Vec<crate::domain::entities::blood_unit::BloodUnit>,
}

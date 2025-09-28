use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Search Units
#[derive(Debug, Deserialize)]
pub struct SearchUnitsQuery {
    pub blood_group: Option<String>,
    pub component_code: Option<String>,
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct SearchUnitsResponse {
    pub units: Vec<crate::domain::entities::blood_unit::BloodUnit>,
    pub total: usize,
    pub limit: i64,
    pub offset: i64,
}

// Get Unit Details
#[derive(Debug, Serialize)]
pub struct GetUnitResponse {
    pub unit: crate::domain::entities::blood_unit::BloodUnit,
    pub donation: Option<crate::domain::entities::donation::BloodDonation>,
    pub donor: Option<crate::domain::entities::donor::BloodDonor>,
}

// Update Unit Status
#[derive(Debug, Deserialize)]
pub struct UpdateUnitStatusRequest {
    pub status: String,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateUnitStatusResponse {
    pub message: String,
}

// Check Compatibility
#[derive(Debug, Deserialize)]
pub struct CheckCompatibilityRequest {
    pub patient_blood_group: String,
    pub unit_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct CheckCompatibilityResponse {
    pub compatible: bool,
    pub reason: Option<String>,
}

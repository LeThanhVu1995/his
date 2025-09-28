use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Perform Crossmatch
#[derive(Debug, Deserialize)]
pub struct PerformCrossmatchRequest {
    pub patient_id: Uuid,
    pub unit_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct PerformCrossmatchResponse {
    pub crossmatch_id: Uuid,
    pub message: String,
}

// Find Compatible Units
#[derive(Debug, Deserialize)]
pub struct FindCompatibleRequest {
    pub patient_id: Uuid,
    pub blood_group: String,
    pub component_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FindCompatibleResponse {
    pub total_found: usize,
    pub compatible_unit_ids: Vec<Uuid>,
    pub message: String,
}

// List Crossmatches
#[derive(Debug, Deserialize)]
pub struct ListCrossmatchesQuery {
    pub patient_id: Option<Uuid>,
    pub unit_id: Option<Uuid>,
    pub result_code: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ListCrossmatchesResponse {
    pub crossmatches: Vec<crate::domain::entities::crossmatch::Crossmatch>,
    pub total: usize,
    pub limit: i64,
    pub offset: i64,
}

// Get Crossmatch
#[derive(Debug, Serialize)]
pub struct GetCrossmatchResponse {
    pub crossmatch: crate::domain::entities::crossmatch::Crossmatch,
    pub unit: Option<crate::domain::entities::blood_unit::BloodUnit>,
}

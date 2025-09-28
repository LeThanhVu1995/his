use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Create Blood Request
#[derive(Debug, Deserialize)]
pub struct CreateBloodRequestRequest {
    pub patient_id: Uuid,
    pub encounter_id: Option<Uuid>,
    pub ordering_provider: Option<Uuid>,
    pub blood_group: String,
    pub component_code: String,
    pub quantity: i32,
    pub priority: String, // STAT, URGENT, ROUTINE
    pub indication: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateBloodRequestResponse {
    pub request_id: Uuid,
    pub message: String,
}

// List Blood Requests
#[derive(Debug, Deserialize)]
pub struct ListBloodRequestsQuery {
    pub patient_id: Option<Uuid>,
    pub encounter_id: Option<Uuid>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ListBloodRequestsResponse {
    pub requests: Vec<crate::domain::entities::blood_request::BloodRequest>,
    pub total: usize,
    pub limit: i64,
    pub offset: i64,
}

// Get Blood Request
#[derive(Debug, Serialize)]
pub struct GetBloodRequestResponse {
    pub request: crate::domain::entities::blood_request::BloodRequest,
}

// Update Blood Request Status
#[derive(Debug, Deserialize)]
pub struct UpdateBloodRequestStatusRequest {
    pub status: String,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UpdateBloodRequestStatusResponse {
    pub message: String,
}

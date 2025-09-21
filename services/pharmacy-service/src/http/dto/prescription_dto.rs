use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreatePrescriptionReq {
    pub patient_id: Uuid,
    pub encounter_id: Option<Uuid>,
    pub note: Option<String>,
    pub items: Vec<CreatePrescItemReq>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreatePrescItemReq {
    pub medication_id: Uuid,
    pub dose: Option<String>,
    pub freq: Option<String>,
    pub duration: Option<String>,
    pub qty: f64,
    pub instruction: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdatePrescriptionReq {
    pub note: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PrescriptionRes {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub encounter_id: Option<Uuid>,
    pub presc_no: String,
    pub status: String,
    pub ordered_by: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PrescriptionQuery {
    pub patient_id: Option<Uuid>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

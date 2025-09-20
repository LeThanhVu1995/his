use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct CreateEncounterReq {
    pub patient_id: Uuid,
    #[validate(length(min = 1, message = "Encounter number is required"))]
    pub encounter_no: String,
    #[validate(length(min = 1, message = "Encounter type is required"))]
    pub encounter_type: String, // OPD/IPD/ER
    pub department_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct UpdateEncounterReq {
    pub status: Option<String>,
    pub department_code: Option<String>,
    pub attending_doctor_id: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EncounterRes {
    pub id: Uuid,
    pub encounter_no: String,
    pub encounter_type: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct EncounterQuery {
    pub patient_id: Option<Uuid>,
    pub status: Option<String>,
    #[validate(range(min = 1, message = "Page must be at least 1"))]
    pub page: Option<i64>,
    #[validate(range(min = 1, max = 200, message = "Page size must be between 1 and 200"))]
    pub page_size: Option<i64>
}

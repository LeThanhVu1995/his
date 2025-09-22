use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BookApptReq {
    pub patient_id: Uuid,
    pub slot_id: Uuid,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CancelApptReq {
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RescheduleApptReq {
    pub new_slot_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct ApptQuery {
    pub patient_id: Option<Uuid>,
    pub provider_id: Option<Uuid>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApptRes {
    pub id: Uuid,
    pub appt_no: String,
    pub status: String,
}

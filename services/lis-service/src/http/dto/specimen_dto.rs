use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateSpecimenReq {
    pub order_id: Option<Uuid>,
    pub patient_id: Uuid,
    pub encounter_id: Option<Uuid>,
    pub specimen_type: String,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SpecimenRes {
    pub id: Uuid,
    pub specimen_no: String,
    pub status: String,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct SpecimenQuery {
    pub patient_id: Option<Uuid>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

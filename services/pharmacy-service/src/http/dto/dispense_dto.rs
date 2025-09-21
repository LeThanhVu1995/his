use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateDispenseReq {
    pub prescription_id: Uuid,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DispenseRes {
    pub id: Uuid,
    pub prescription_id: Uuid,
    pub disp_no: String,
    pub dispensed_by: Option<String>,
    pub status: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct DispenseQuery {
    pub prescription_id: Option<Uuid>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

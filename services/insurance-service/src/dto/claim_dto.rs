use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ClaimItemReq {
    pub code: String,
    pub qty: f64,
    pub unit_price: f64,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateClaimReq {
    pub patient_id: Uuid,
    pub encounter_id: Option<Uuid>,
    pub member_id: Uuid,
    pub payer: String,
    pub items: Vec<ClaimItemReq>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ClaimRes {
    pub id: Uuid,
    pub claim_no: String,
    pub status: String,
}

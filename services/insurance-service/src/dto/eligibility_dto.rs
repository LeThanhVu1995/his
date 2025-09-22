use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct EligibilityReqDto {
    pub payer: String,
    pub policy_no: String,
    pub patient_id: Uuid,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EligibilityResDto {
    pub eligible: bool,
    pub member_id: Uuid,
    pub plan_code: Option<String>,
}

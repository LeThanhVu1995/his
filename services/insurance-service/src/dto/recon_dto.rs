use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateReconReq {
    pub payer: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_claims: i64,
    pub total_amount: f64,
    pub approved_amount: f64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReconRes {
    pub id: Uuid,
    pub batch_no: String,
}

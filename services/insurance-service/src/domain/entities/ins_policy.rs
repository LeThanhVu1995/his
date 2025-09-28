use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::NaiveDate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct InsPolicy {
    pub policy_id: String,
    pub patient_id: String,
    pub payer_id: String,
    pub policy_no: String,
    pub coverage_json: Option<String>,
    pub valid_from: Option<NaiveDate>,
    pub valid_to: Option<NaiveDate>,
    pub status: String,
}

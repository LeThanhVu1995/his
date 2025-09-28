use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateInsPolicyRequest {
    pub patient_id: String,
    pub payer_id: String,
    pub policy_no: String,
    pub coverage_json: Option<String>,
    pub valid_from: Option<NaiveDate>,
    pub valid_to: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateInsPolicyRequest {
    pub patient_id: Option<String>,
    pub payer_id: Option<String>,
    pub policy_no: Option<String>,
    pub coverage_json: Option<String>,
    pub valid_from: Option<NaiveDate>,
    pub valid_to: Option<NaiveDate>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InsPolicyResponse {
    pub policy_id: String,
    pub patient_id: String,
    pub payer_id: String,
    pub policy_no: String,
    pub coverage_json: Option<String>,
    pub valid_from: Option<NaiveDate>,
    pub valid_to: Option<NaiveDate>,
    pub status: String,
}

impl InsPolicyResponse {
    pub fn from_entity(policy: &crate::domain::entities::ins_policy::InsPolicy) -> Self {
        Self {
            policy_id: policy.policy_id.clone(),
            patient_id: policy.patient_id.clone(),
            payer_id: policy.payer_id.clone(),
            policy_no: policy.policy_no.clone(),
            coverage_json: policy.coverage_json.clone(),
            valid_from: policy.valid_from,
            valid_to: policy.valid_to,
            status: policy.status.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListInsPoliciesRequest {
    pub patient_id: Option<String>,
    pub payer_id: Option<String>,
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListInsPoliciesResponse {
    pub policies: Vec<InsPolicyResponse>,
    pub total: i64,
}

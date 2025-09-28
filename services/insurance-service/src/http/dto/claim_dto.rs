use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateInsClaimRequest {
    pub encounter_id: String,
    pub policy_id: String,
    pub items: Vec<CreateInsClaimItemRequest>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateInsClaimItemRequest {
    pub service_code: String,
    pub description: Option<String>,
    pub qty: Option<f64>,
    pub unit_price: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateInsClaimRequest {
    pub encounter_id: Option<String>,
    pub policy_id: Option<String>,
    pub status: Option<String>,
    pub total_claimed: Option<f64>,
    pub total_approved: Option<f64>,
    pub response_code: Option<String>,
    pub response_text: Option<String>,
    pub signature_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InsClaimResponse {
    pub claim_id: String,
    pub encounter_id: String,
    pub policy_id: String,
    pub status: String,
    pub total_claimed: Option<f64>,
    pub total_approved: Option<f64>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub response_at: Option<DateTime<Utc>>,
    pub response_code: Option<String>,
    pub response_text: Option<String>,
    pub signature_id: Option<String>,
    pub items: Vec<InsClaimItemResponse>,
}

impl InsClaimResponse {
    pub fn from_entity(claim: &crate::domain::entities::claim::InsClaim) -> Self {
        Self {
            claim_id: claim.claim_id.clone(),
            encounter_id: claim.encounter_id.clone(),
            policy_id: claim.policy_id.clone(),
            status: claim.status.clone(),
            total_claimed: claim.total_claimed,
            total_approved: claim.total_approved,
            submitted_at: claim.submitted_at,
            response_at: claim.response_at,
            response_code: claim.response_code.clone(),
            response_text: claim.response_text.clone(),
            signature_id: claim.signature_id.clone(),
            items: vec![], // Will be populated separately
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct InsClaimItemResponse {
    pub claim_item_id: String,
    pub claim_id: String,
    pub service_code: String,
    pub description: Option<String>,
    pub qty: Option<f64>,
    pub unit_price: Option<f64>,
    pub amount: Option<f64>,
    pub approved_amount: Option<f64>,
}

impl InsClaimItemResponse {
    pub fn from_entity(item: &crate::domain::entities::claim::InsClaimItem) -> Self {
        Self {
            claim_item_id: item.claim_item_id.clone(),
            claim_id: item.claim_id.clone(),
            service_code: item.service_code.clone(),
            description: item.description.clone(),
            qty: item.qty,
            unit_price: item.unit_price,
            amount: item.amount,
            approved_amount: item.approved_amount,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListInsClaimsRequest {
    pub encounter_id: Option<String>,
    pub policy_id: Option<String>,
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListInsClaimsResponse {
    pub claims: Vec<InsClaimResponse>,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SubmitInsClaimRequest {
    pub claim_id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SignInsClaimRequest {
    pub claim_id: String,
    pub signature_id: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateInsClaimStatusRequest {
    pub status: String,
}

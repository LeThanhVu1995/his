use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct InsClaim {
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
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct InsClaimItem {
    pub claim_item_id: String,
    pub claim_id: String,
    pub service_code: String,
    pub description: Option<String>,
    pub qty: Option<f64>,
    pub unit_price: Option<f64>,
    pub amount: Option<f64>,
    pub approved_amount: Option<f64>,
}

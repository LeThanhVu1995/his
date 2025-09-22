use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Claim {
    pub id: Uuid,
    pub claim_no: String,
    pub patient_id: Uuid,
    pub encounter_id: Option<Uuid>,
    pub member_id: Uuid,
    pub payer: String,
    pub total_amount: f64,
    pub currency: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct ClaimItem {
    pub id: Uuid,
    pub claim_id: Uuid,
    pub code: String,
    pub description: Option<String>,
    pub qty: f64,
    pub unit_price: f64,
    pub amount: f64,
    pub coverage_rate: Option<f64>,
    pub patient_pay: Option<f64>,
}

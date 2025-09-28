use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Payment {
    pub payment_id: Uuid,
    pub invoice_id: Uuid,
    pub method_code: String, // CASH, CARD, BANK, INSURANCE
    pub amount: f64,
    pub paid_at: DateTime<Utc>,
    pub ref_no: Option<String>,
    pub status: String, // PENDING, COMPLETED, FAILED, REFUNDED
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct PaymentAllocation {
    pub allocation_id: Uuid,
    pub payment_id: Uuid,
    pub invoice_item_id: Uuid,
    pub amount: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Refund {
    pub refund_id: Uuid,
    pub payment_id: Uuid,
    pub amount: f64,
    pub reason: Option<String>,
    pub refunded_at: DateTime<Utc>,
    pub ref_no: Option<String>,
    pub status: String, // PENDING, COMPLETED, FAILED
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateRefundReq {
    pub payment_id: Uuid,
    pub amount: f64,
    pub reason: Option<String>,
    pub ref_no: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RefundRes {
    pub id: Uuid,
    pub payment_id: Uuid,
    pub amount: f64,
    pub reason: Option<String>,
    pub status: String,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct RefundQuery {
    pub payment_id: Option<Uuid>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

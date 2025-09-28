use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreatePaymentReq {
    pub invoice_id: Uuid,
    pub method_code: String,
    pub amount: f64,
    pub ref_no: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaymentRes {
    pub id: Uuid,
    pub amount: f64,
    pub method_code: String,
    pub status: String,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct PaymentQuery {
    pub invoice_id: Option<Uuid>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

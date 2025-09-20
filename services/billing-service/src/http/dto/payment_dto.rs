use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use utoipa::ToSchema;
use bigdecimal::BigDecimal;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreatePaymentReq {
    pub invoice_id: Uuid,
    pub method: String,
    pub amount: BigDecimal,
    pub currency: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PaymentRes {
    pub id: Uuid,
    pub pay_no: String,
    pub amount: BigDecimal,
    pub method: String,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct PaymentQuery {
    pub invoice_id: Option<Uuid>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

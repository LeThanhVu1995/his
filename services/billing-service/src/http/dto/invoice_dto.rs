use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use utoipa::ToSchema;
use bigdecimal::BigDecimal;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateInvoiceReq {
    pub patient_id: Uuid,
    pub encounter_id: Option<Uuid>,
    pub charge_ids: Vec<Uuid>,
    pub discount: Option<BigDecimal>,
    pub tax: Option<BigDecimal>,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct InvoiceRes {
    pub id: Uuid,
    pub invoice_no: String,
    pub total: BigDecimal,
    pub status: String,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct InvoiceQuery {
    pub encounter_id: Option<Uuid>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

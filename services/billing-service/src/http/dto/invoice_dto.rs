use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateInvoiceReq {
    pub patient_id: Uuid,
    pub encounter_id: Uuid,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct InvoiceRes {
    pub id: Uuid,
    pub invoice_no: String,
    pub total: f64,
    pub status: String,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct InvoiceQuery {
    pub encounter_id: Option<Uuid>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

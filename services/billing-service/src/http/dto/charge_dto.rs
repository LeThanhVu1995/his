use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use utoipa::ToSchema;
use bigdecimal::BigDecimal;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateChargeReq {
    pub patient_id: Uuid,
    pub encounter_id: Option<Uuid>,
    pub order_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub qty: BigDecimal,
    pub unit_price: BigDecimal,
    pub currency: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateChargeReq {
    pub name: Option<String>,
    pub qty: Option<f64>,
    pub unit_price: Option<f64>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct ChargeQuery {
    pub encounter_id: Option<Uuid>,
    pub status: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ChargeRes {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub qty: BigDecimal,
    pub unit_price: BigDecimal,
    pub amount: BigDecimal,
    pub status: String,
}

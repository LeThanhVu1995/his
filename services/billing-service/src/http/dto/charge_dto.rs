use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateChargeReq {
    pub patient_id: Uuid,
    pub encounter_id: Uuid,
    pub code: String,
    pub description: Option<String>,
    pub qty: f64,
    pub unit_price: f64,
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
    pub qty: f64,
    pub unit_price: f64,
    pub amount: f64,
    pub status: String,
}

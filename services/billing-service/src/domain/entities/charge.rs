use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct PriceList {
    pub price_list_id: Uuid,
    pub facility_id: Uuid,
    pub code: String,
    pub name: String,
    pub currency: String,
    pub valid_from: Option<chrono::NaiveDate>,
    pub valid_to: Option<chrono::NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct PriceItem {
    pub price_item_id: Uuid,
    pub price_list_id: Uuid,
    pub service_code: String,
    pub description: Option<String>,
    pub unit_price: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Charge {
    pub charge_id: Uuid,
    pub encounter_id: Uuid,
    pub patient_id: Uuid,
    pub service_code: String,
    pub description: Option<String>,
    pub qty: f64,
    pub unit_price: f64,
    pub amount: f64,
    pub status: String, // PENDING, INVOICED, CANCELLED
    pub charged_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

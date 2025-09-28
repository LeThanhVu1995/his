use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct InvoiceItem {
    pub invoice_item_id: Uuid,
    pub invoice_id: Uuid,
    pub service_code: String,
    pub description: Option<String>,
    pub qty: f64,
    pub unit_price: f64,
    pub amount: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


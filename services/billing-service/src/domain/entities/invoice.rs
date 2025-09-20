use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use bigdecimal::BigDecimal;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Invoice {
    pub id: Uuid,
    pub invoice_no: String,
    pub patient_id: Uuid,
    pub encounter_id: Option<Uuid>,
    pub subtotal: BigDecimal,
    pub discount: BigDecimal,
    pub tax: BigDecimal,
    pub total: BigDecimal,
    pub status: String,
    pub note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

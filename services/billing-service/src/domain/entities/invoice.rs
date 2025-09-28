use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Invoice {
    pub invoice_id: Uuid,
    pub encounter_id: Uuid,
    pub patient_id: Uuid,
    pub status: String, // OPEN, PAID, CANCELLED, REFUNDED
    pub total_amount: f64,
    pub currency: String,
    pub issued_at: DateTime<Utc>,
    pub due_date: Option<chrono::NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

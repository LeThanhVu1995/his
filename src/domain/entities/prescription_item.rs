use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use rust_decimal::Decimal;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct PrescriptionItem {
    pub id: Uuid,
    pub prescription_id: Uuid,
    pub medication_id: Uuid,
    pub dose: Option<String>,
    pub freq: Option<String>,
    pub duration: Option<String>,
    pub qty: Decimal,
    pub instruction: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

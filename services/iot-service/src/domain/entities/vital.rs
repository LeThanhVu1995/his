use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Vital {
    pub id: Uuid,
    pub device_id: Uuid,
    pub patient_id: Option<Uuid>,
    pub encounter_id: Option<Uuid>,
    pub vital_type: String,
    pub value: Decimal,
    pub unit: String,
    pub measured_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

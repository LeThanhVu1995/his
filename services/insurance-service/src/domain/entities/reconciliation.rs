use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Reconciliation {
    pub id: Uuid,
    pub batch_no: String,
    pub payer: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_claims: i64,
    pub total_amount: f64,
    pub approved_amount: f64,
    pub created_at: DateTime<Utc>,
}

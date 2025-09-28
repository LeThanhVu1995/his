use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Observation {
    pub obs_id: Uuid,
    pub encounter_id: Uuid,
    pub patient_id: Uuid,
    pub device_id: Option<Uuid>,
    pub code: String,
    pub value_num: Option<Decimal>,
    pub value_text: Option<String>,
    pub unit: Option<String>,
    pub taken_at: DateTime<Utc>,
    pub performer_staff_id: Option<Uuid>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, validator::Validate)]
pub struct CreateObservationRequest {
    pub encounter_id: Uuid,
    pub patient_id: Uuid,
    pub device_id: Option<Uuid>,
    pub code: String,
    pub value_num: Option<Decimal>,
    pub value_text: Option<String>,
    pub unit: Option<String>,
    pub taken_at: Option<DateTime<Utc>>,
    pub performer_staff_id: Option<Uuid>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, validator::Validate)]
pub struct UpdateObservationRequest {
    pub value_num: Option<Decimal>,
    pub value_text: Option<String>,
    pub unit: Option<String>,
    pub taken_at: Option<DateTime<Utc>>,
    pub performer_staff_id: Option<Uuid>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ObservationResponse {
    pub obs_id: Uuid,
    pub encounter_id: Uuid,
    pub patient_id: Uuid,
    pub device_id: Option<Uuid>,
    pub code: String,
    pub value_num: Option<Decimal>,
    pub value_text: Option<String>,
    pub unit: Option<String>,
    pub taken_at: DateTime<Utc>,
    pub performer_staff_id: Option<Uuid>,
    pub status: String,
}

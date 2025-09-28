use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct VitalSignRecord {
    pub vs_id: Uuid,
    pub encounter_id: Uuid,
    pub patient_id: Uuid,
    pub device_id: Option<Uuid>,
    pub measured_at: DateTime<Utc>,
    pub recorder_staff_id: Option<Uuid>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct VitalSignItem {
    pub vs_item_id: Uuid,
    pub vs_id: Uuid,
    pub code: String,
    pub value_num: Option<Decimal>,
    pub value_text: Option<String>,
    pub unit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, validator::Validate)]
pub struct CreateVitalSignRequest {
    pub encounter_id: Uuid,
    pub patient_id: Uuid,
    pub device_id: Option<Uuid>,
    pub measured_at: Option<DateTime<Utc>>,
    pub recorder_staff_id: Option<Uuid>,
    pub note: Option<String>,
    pub items: Vec<CreateVitalSignItemRequest>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateVitalSignItemRequest {
    pub code: String,
    pub value_num: Option<Decimal>,
    pub value_text: Option<String>,
    pub unit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct VitalSignResponse {
    pub vs_id: Uuid,
    pub encounter_id: Uuid,
    pub patient_id: Uuid,
    pub device_id: Option<Uuid>,
    pub measured_at: DateTime<Utc>,
    pub recorder_staff_id: Option<Uuid>,
    pub note: Option<String>,
    pub items: Vec<VitalSignItem>,
}

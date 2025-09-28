use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct DeviceReading {
    pub reading_id: Uuid,
    pub device_id: Uuid,
    pub sensor_type: String,
    pub value_num: Option<Decimal>,
    pub value_text: Option<String>,
    pub unit: Option<String>,
    pub quality: String,
    pub read_at: DateTime<Utc>,
    pub raw_data: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, validator::Validate)]
pub struct CreateDeviceReadingRequest {
    pub device_id: Uuid,
    pub sensor_type: String,
    pub value_num: Option<Decimal>,
    pub value_text: Option<String>,
    pub unit: Option<String>,
    pub quality: Option<String>,
    pub read_at: Option<DateTime<Utc>>,
    pub raw_data: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeviceReadingResponse {
    pub reading_id: Uuid,
    pub device_id: Uuid,
    pub sensor_type: String,
    pub value_num: Option<Decimal>,
    pub value_text: Option<String>,
    pub unit: Option<String>,
    pub quality: String,
    pub read_at: DateTime<Utc>,
    pub raw_data: Option<Value>,
}

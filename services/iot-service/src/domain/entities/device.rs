use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Device {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    #[sqlx(rename = "type")]
    pub r#type: String,
    pub location: Option<String>,
    pub last_seen: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, validator::Validate)]
pub struct CreateDeviceRequest {
    pub code: String,
    pub name: String,
    pub r#type: String,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, validator::Validate)]
pub struct UpdateDeviceRequest {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub location: Option<String>,
}

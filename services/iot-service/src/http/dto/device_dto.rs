use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpsertDeviceReq {
    pub code: String,
    pub name: String,
    pub r#type: String,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct DeviceResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub r#type: String,
    pub location: Option<String>,
    pub last_seen: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

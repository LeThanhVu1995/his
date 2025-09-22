use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
pub struct SlotQuery {
    pub provider_id: Uuid,
    pub date_from: DateTime<Utc>,
    pub date_to: DateTime<Utc>,
    pub only_free: Option<bool>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct GenerateSlotsReq {
    pub provider_id: Uuid,
    pub date_from: DateTime<Utc>,
    pub date_to: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SlotRes {
    pub id: Uuid,
    pub provider_id: Uuid,
    pub room_id: Option<Uuid>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub reserved: bool,
}

use chrono::{DateTime, Utc, NaiveTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Provider {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub specialty: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Room {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub location: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Schedule {
    pub id: Uuid,
    pub provider_id: Uuid,
    pub room_id: Option<Uuid>,
    pub weekday: i16,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub slot_min: i16,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct TimeSlot {
    pub id: Uuid,
    pub provider_id: Uuid,
    pub room_id: Option<Uuid>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub reserved: bool,
    pub locked_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Appointment {
    pub id: Uuid,
    pub appt_no: String,
    pub patient_id: Uuid,
    pub provider_id: Uuid,
    pub room_id: Option<Uuid>,
    pub slot_id: Uuid,
    pub status: String,
    pub reason: Option<String>,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

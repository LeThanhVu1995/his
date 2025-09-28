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
pub struct ResourceEquipment {
    pub equipment_id: Uuid,
    pub facility_id: Uuid,
    pub department_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub type_code: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Appointment {
    pub id: Uuid,
    pub appt_no: String,
    pub patient_id: Uuid,
    pub provider_id: Uuid,
    pub room_id: Option<Uuid>,
    pub slot_id: Uuid,
    pub facility_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub staff_id: Option<Uuid>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub status: String,
    pub reason: Option<String>,
    pub reason_text: Option<String>,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

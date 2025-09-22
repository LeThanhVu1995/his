use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::NaiveTime;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateScheduleReq {
    pub provider_id: Uuid,
    pub room_id: Option<Uuid>,
    pub weekday: i16,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub slot_min: i16,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateScheduleReq {
    pub room_id: Option<Uuid>,
    pub weekday: Option<i16>,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
    pub slot_min: Option<i16>,
    pub active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ScheduleQuery {
    pub provider_id: Option<Uuid>,
    pub weekday: Option<i16>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ScheduleRes {
    pub id: Uuid,
    pub provider_id: Uuid,
    pub room_id: Option<Uuid>,
    pub weekday: i16,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub slot_min: i16,
    pub active: bool,
}

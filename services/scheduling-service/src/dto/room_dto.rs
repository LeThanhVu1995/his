use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateRoomReq {
    pub code: String,
    pub name: String,
    pub location: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateRoomReq {
    pub name: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RoomQuery {
    pub q: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RoomRes {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub location: Option<String>,
}

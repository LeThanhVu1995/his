use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MovementDto {
    pub id: Uuid,
    pub mv_no: String,
    pub mv_type: String,
    pub src_wh: Option<Uuid>,
    pub dst_wh: Option<Uuid>,
    pub note: Option<String>,
    pub created_by: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateMovementDto {
    pub mv_no: String,
    pub mv_type: String,
    pub src_wh: Option<Uuid>,
    pub dst_wh: Option<Uuid>,
    pub note: Option<String>,
    pub created_by: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateMovementDto {
    pub mv_no: Option<String>,
    pub mv_type: Option<String>,
    pub src_wh: Option<Uuid>,
    pub dst_wh: Option<Uuid>,
    pub note: Option<String>,
}

// Legacy structs for backward compatibility
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ReceiveReq {
    pub dst_wh: Uuid,
    pub lines: Vec<MoveLineReq>,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct IssueReq {
    pub src_wh: Uuid,
    pub lines: Vec<MoveLineReq>,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TransferReq {
    pub src_wh: Uuid,
    pub dst_wh: Uuid,
    pub lines: Vec<MoveLineReq>,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AdjustReq {
    pub wh: Uuid,
    pub lines: Vec<AdjustLineReq>,
    pub note: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema, Clone)]
pub struct MoveLineReq {
    pub item_id: Uuid,
    pub lot_id: Option<Uuid>,
    pub qty: f64,
}

#[derive(Debug, Deserialize, Validate, ToSchema, Clone)]
pub struct AdjustLineReq {
    pub item_id: Uuid,
    pub lot_id: Option<Uuid>,
    pub diff: f64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MovementRes {
    pub id: Uuid,
    pub mv_no: String,
    pub mv_type: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct MovementQuery {
    pub mv_type: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

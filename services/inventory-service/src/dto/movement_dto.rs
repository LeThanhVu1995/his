use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;

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

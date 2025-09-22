use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::NaiveDate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateLotReq {
    pub item_id: Uuid,
    pub lot_no: String,
    pub exp_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LotRes {
    pub id: Uuid,
    pub item_id: Uuid,
    pub lot_no: String,
    pub exp_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LotQuery {
    pub item_id: Option<Uuid>,
    pub q: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

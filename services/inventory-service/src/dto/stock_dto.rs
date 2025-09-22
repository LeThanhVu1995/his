use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct StockRes {
    pub warehouse_id: Uuid,
    pub item_id: Uuid,
    pub lot_id: Option<Uuid>,
    pub qty: f64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct StockQuery {
    pub warehouse_id: Option<Uuid>,
    pub item_id: Option<Uuid>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

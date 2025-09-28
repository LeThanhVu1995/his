use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use rust_decimal::Decimal;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StockTransactionDto {
    pub id: Uuid,
    pub warehouse_id: Uuid,
    pub item_id: Uuid,
    pub batch_id: Option<Uuid>,
    pub qty_delta: Decimal,
    pub uom_id: Uuid,
    pub reason_code: String,
    pub ref_entity: Option<String>,
    pub ref_id: Option<Uuid>,
    pub occurred_at: DateTime<Utc>,
    pub performed_by: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct StockTransactionQuery {
    pub warehouse_id: Option<Uuid>,
    pub item_id: Option<Uuid>,
    pub reason_code: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateStockTransactionDto {
    pub warehouse_id: Uuid,
    pub item_id: Uuid,
    pub batch_id: Option<Uuid>,
    pub qty_delta: Decimal,
    pub uom_id: Uuid,
    pub reason_code: String,
    pub ref_entity: Option<String>,
    pub ref_id: Option<Uuid>,
    pub performed_by: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StockTransactionFilterDto {
    pub warehouse_id: Option<Uuid>,
    pub item_id: Option<Uuid>,
    pub batch_id: Option<Uuid>,
    pub reason_code: Option<String>,
    pub ref_entity: Option<String>,
    pub ref_id: Option<Uuid>,
    pub from_date: Option<DateTime<Utc>>,
    pub to_date: Option<DateTime<Utc>>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

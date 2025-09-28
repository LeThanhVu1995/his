use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use rust_decimal::Decimal;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GoodsReceiptDto {
    pub id: Uuid,
    pub po_id: Option<Uuid>,
    pub warehouse_id: Uuid,
    pub grn_no: String,
    pub received_at: DateTime<Utc>,
    pub received_by: Option<String>,
    pub status: String,
    pub total_amount: Decimal,
    pub currency: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub items: Option<Vec<GoodsReceiptItemDto>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateGoodsReceiptDto {
    pub po_id: Option<Uuid>,
    pub warehouse_id: Uuid,
    #[validate(length(max = 100))]
    pub received_by: Option<String>,
    #[validate(length(max = 3))]
    pub currency: Option<String>,
    #[validate(length(max = 1000))]
    pub notes: Option<String>,
    #[validate(length(min = 1))]
    pub items: Vec<CreateGoodsReceiptItemDto>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateGoodsReceiptDto {
    #[validate(length(max = 100))]
    pub received_by: Option<String>,
    #[validate(length(max = 3))]
    pub currency: Option<String>,
    #[validate(length(max = 1000))]
    pub notes: Option<String>,
    #[validate(length(max = 32))]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GoodsReceiptItemDto {
    pub id: Uuid,
    pub grn_id: Uuid,
    pub item_id: Uuid,
    pub batch_id: Option<Uuid>,
    pub quantity: Decimal,
    pub uom_id: Uuid,
    pub unit_price: Option<Decimal>,
    pub total_price: Option<Decimal>,
    pub expiry_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateGoodsReceiptItemDto {
    pub item_id: Uuid,
    pub batch_id: Option<Uuid>,
    pub quantity: Decimal,
    pub uom_id: Uuid,
    pub unit_price: Option<Decimal>,
    pub expiry_date: Option<NaiveDate>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateGoodsReceiptItemDto {
    pub batch_id: Option<Uuid>,
    pub quantity: Option<Decimal>,
    pub uom_id: Option<Uuid>,
    pub unit_price: Option<Decimal>,
    pub expiry_date: Option<NaiveDate>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct GoodsReceiptQuery {
    pub q: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

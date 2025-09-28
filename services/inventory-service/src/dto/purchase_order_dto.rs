use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use rust_decimal::Decimal;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PurchaseOrderDto {
    pub id: Uuid,
    pub supplier_id: Uuid,
    pub facility_id: Uuid,
    pub po_no: String,
    pub status: String,
    pub ordered_at: DateTime<Utc>,
    pub expected_delivery_date: Option<NaiveDate>,
    pub total_amount: Decimal,
    pub currency: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub items: Option<Vec<PurchaseOrderItemDto>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreatePurchaseOrderDto {
    pub supplier_id: Uuid,
    pub facility_id: Uuid,
    pub expected_delivery_date: Option<NaiveDate>,
    #[validate(length(max = 3))]
    pub currency: Option<String>,
    #[validate(length(max = 1000))]
    pub notes: Option<String>,
    #[validate(length(min = 1))]
    pub items: Vec<CreatePurchaseOrderItemDto>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdatePurchaseOrderDto {
    pub expected_delivery_date: Option<NaiveDate>,
    #[validate(length(max = 3))]
    pub currency: Option<String>,
    #[validate(length(max = 1000))]
    pub notes: Option<String>,
    #[validate(length(max = 32))]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PurchaseOrderItemDto {
    pub id: Uuid,
    pub po_id: Uuid,
    pub item_id: Uuid,
    pub quantity: Decimal,
    pub uom_id: Uuid,
    pub unit_price: Decimal,
    pub total_price: Decimal,
    pub received_quantity: Decimal,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreatePurchaseOrderItemDto {
    pub item_id: Uuid,
    pub quantity: Decimal,
    pub uom_id: Uuid,
    pub unit_price: Decimal,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdatePurchaseOrderItemDto {
    pub quantity: Option<Decimal>,
    pub uom_id: Option<Uuid>,
    pub unit_price: Option<Decimal>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct PurchaseOrderQuery {
    pub q: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

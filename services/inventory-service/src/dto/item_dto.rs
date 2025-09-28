use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ItemDto {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub uom: String,
    pub base_uom_id: Option<Uuid>,
    pub category_code: Option<String>,
    pub is_med: bool,
    pub is_consumable: bool,
    pub is_lot_tracked: bool,
    pub is_expirable: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateItemDto {
    pub code: String,
    pub name: String,
    pub uom: String,
    pub base_uom_id: Option<Uuid>,
    pub category_code: Option<String>,
    pub is_med: Option<bool>,
    pub is_consumable: Option<bool>,
    pub is_lot_tracked: Option<bool>,
    pub is_expirable: Option<bool>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateItemDto {
    pub name: Option<String>,
    pub uom: Option<String>,
    pub base_uom_id: Option<Uuid>,
    pub category_code: Option<String>,
    pub is_med: Option<bool>,
    pub is_consumable: Option<bool>,
    pub is_lot_tracked: Option<bool>,
    pub is_expirable: Option<bool>,
}

// Legacy structs for backward compatibility
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateItemReq {
    pub code: String,
    pub name: String,
    pub uom: String,
    pub is_med: Option<bool>,
    pub is_consumable: Option<bool>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateItemReq {
    pub name: Option<String>,
    pub uom: Option<String>,
    pub is_consumable: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ItemRes {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub uom: String,
    pub is_med: bool,
    pub is_consumable: bool,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ItemQuery {
    pub q: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

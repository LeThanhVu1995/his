use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use rust_decimal::Decimal;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UomDto {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateUomDto {
    #[validate(length(min = 1, max = 32))]
    pub code: String,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateUomDto {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UomQuery {
    pub q: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ItemUomDto {
    pub id: Uuid,
    pub item_id: Uuid,
    pub uom_id: Uuid,
    pub factor: Decimal,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateItemUomDto {
    pub item_id: Uuid,
    pub uom_id: Uuid,
    pub factor: Decimal,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateItemUomDto {
    pub factor: Option<Decimal>,
}

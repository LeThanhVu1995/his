use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WarehouseDto {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub r#type: String,
    pub facility_id: Option<Uuid>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateWarehouseDto {
    pub code: String,
    pub name: String,
    pub r#type: Option<String>,
    pub facility_id: Option<Uuid>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateWarehouseDto {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub facility_id: Option<Uuid>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub status: Option<String>,
}

// Legacy structs for backward compatibility
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateWarehouseReq {
    pub code: String,
    pub name: String,
    pub r#type: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateWarehouseReq {
    pub name: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WarehouseRes {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct WarehouseQuery {
    pub q: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

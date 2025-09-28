use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SupplierDto {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address_line1: Option<String>,
    pub address_line2: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub tax_id: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateSupplierDto {
    #[validate(length(min = 1, max = 64))]
    pub code: String,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(max = 32))]
    pub phone: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(max = 255))]
    pub address_line1: Option<String>,
    #[validate(length(max = 255))]
    pub address_line2: Option<String>,
    #[validate(length(max = 255))]
    pub city: Option<String>,
    #[validate(length(max = 255))]
    pub province: Option<String>,
    #[validate(length(max = 64))]
    pub country: Option<String>,
    #[validate(length(max = 32))]
    pub postal_code: Option<String>,
    #[validate(length(max = 32))]
    pub tax_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateSupplierDto {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(max = 32))]
    pub phone: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(max = 255))]
    pub address_line1: Option<String>,
    #[validate(length(max = 255))]
    pub address_line2: Option<String>,
    #[validate(length(max = 255))]
    pub city: Option<String>,
    #[validate(length(max = 255))]
    pub province: Option<String>,
    #[validate(length(max = 64))]
    pub country: Option<String>,
    #[validate(length(max = 32))]
    pub postal_code: Option<String>,
    #[validate(length(max = 32))]
    pub tax_id: Option<String>,
    #[validate(length(max = 32))]
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct SupplierQuery {
    pub q: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

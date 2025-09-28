// use serde::{Deserialize, Serialize};
// use utoipa::ToSchema;
// use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct InvUom {
    pub uom_id: String,
    pub code: String,
    pub name: String,
}

// DTOs for API requests
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateUomRequest {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateUomRequest {
    pub name: Option<String>,
}

// Query DTOs
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct UomQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub code: Option<String>,
}

use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;

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

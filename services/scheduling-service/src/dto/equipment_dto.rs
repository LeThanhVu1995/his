use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateEquipmentRequest {
    #[validate(custom = "crate::dto::common::validate_uuid")]
    pub facility_id: String,
    pub department_id: Option<String>,
    #[validate(length(min = 1, max = 64))]
    pub code: String,
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(max = 64))]
    pub type_code: Option<String>,
    #[validate(length(min = 1, max = 32))]
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateEquipmentRequest {
    #[validate(length(min = 1, max = 64))]
    pub code: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    #[validate(length(max = 64))]
    pub type_code: Option<String>,
    #[validate(length(min = 1, max = 32))]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EquipmentQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub facility_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub type_code: Option<String>,
    pub status: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct EquipmentResponse {
    pub equipment_id: Uuid,
    pub facility_id: Uuid,
    pub department_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub type_code: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
}

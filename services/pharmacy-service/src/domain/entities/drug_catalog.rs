use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct DrugCatalog {
    pub drug_id: Uuid,
    pub code: String,
    pub name: String,
    pub generic_name: Option<String>,
    pub form_code: Option<String>,
    pub strength_text: Option<String>,
    pub atc_code: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by: Option<Uuid>,
}

// Request DTOs
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateDrugCatalogRequest {
    #[validate(length(min = 1, message = "Code cannot be empty"))]
    pub code: String,
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
    pub generic_name: Option<String>,
    pub form_code: Option<String>,
    pub strength_text: Option<String>,
    pub atc_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateDrugCatalogRequest {
    #[validate(length(min = 1, message = "Code cannot be empty"))]
    pub code: Option<String>,
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: Option<String>,
    pub generic_name: Option<String>,
    pub form_code: Option<String>,
    pub strength_text: Option<String>,
    pub atc_code: Option<String>,
}

// Query DTOs
#[derive(Debug, Deserialize, ToSchema, Validate, serde::Serialize, sqlx::FromRow)]
pub struct DrugCatalogQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub code: Option<String>,
    pub name: Option<String>,
    pub atc_code: Option<String>,
    pub form_code: Option<String>,
}

// Response DTOs
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct DrugCatalogStats {
    pub total: i64,
    pub by_form: serde_json::Value,
    pub by_atc: serde_json::Value,
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct CreateCodeReq {
    #[validate(length(min = 1, max = 50, message = "Category must be between 1 and 50 characters"))]
    pub category: String,
    #[validate(length(min = 1, max = 20, message = "Code must be between 1 and 20 characters"))]
    pub code: String,
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: String,
    #[validate(length(max = 500, message = "Description must be less than 500 characters"))]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct UpdateCodeReq {
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: Option<String>,
    #[validate(length(max = 500, message = "Description must be less than 500 characters"))]
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CodeRes {
    pub id: Uuid,
    pub category: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ListCodesQuery {
    pub category: Option<String>,
    #[validate(range(min = 1, message = "Page must be at least 1"))]
    pub page: Option<i64>,       // 1-based
    #[validate(range(min = 1, max = 200, message = "Page size must be between 1 and 200"))]
    pub page_size: Option<i64>,  // <= 200
    pub search: Option<String>,  // Search in name/description
    pub is_active: Option<bool>, // Filter by active status
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct BulkCreateCodeReq {
    #[validate(length(min = 1, max = 100, message = "Cannot create more than 100 codes at once"))]
    pub codes: Vec<CreateCodeReq>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct BulkUpdateCodeReq {
    #[validate(length(min = 1, max = 100, message = "Cannot update more than 100 codes at once"))]
    pub updates: Vec<BulkUpdateItem>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct BulkUpdateItem {
    pub id: Uuid,
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: Option<String>,
    #[validate(length(max = 500, message = "Description must be less than 500 characters"))]
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BulkCreateCodeRes {
    pub created: Vec<CodeRes>,
    pub failed: Vec<BulkError>,
    pub total_created: usize,
    pub total_failed: usize,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BulkError {
    pub index: usize,
    pub error: String,
}

pub fn calc_etag(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("\"{:x}\"", hasher.finalize()) // ETag phải có dấu quote
}

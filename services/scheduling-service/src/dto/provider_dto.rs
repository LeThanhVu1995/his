use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateProviderReq {
    pub code: String,
    pub name: String,
    pub specialty: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateProviderReq {
    pub name: Option<String>,
    pub specialty: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ProviderQuery {
    pub q: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProviderRes {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub specialty: Option<String>,
}

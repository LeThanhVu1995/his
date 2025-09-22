use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateTemplateReq {
    pub code: String,
    pub name: String,
    pub channel: String,
    pub subject: Option<String>,
    pub body: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateTemplateReq {
    pub name: Option<String>,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RenderReq {
    pub body: String,
    pub variables: serde_json::Value,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TemplateRes {
    pub id: Uuid,
    pub code: String,
    pub channel: String,
}

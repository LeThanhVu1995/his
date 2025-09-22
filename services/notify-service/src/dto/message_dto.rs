use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SendMessageReq {
    pub channel: String,
    pub to: String,
    pub subject: Option<String>,
    pub body: String,
    pub variables: Option<serde_json::Value>,
    pub template_code: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SendMessageRes {
    pub id: Uuid,
}

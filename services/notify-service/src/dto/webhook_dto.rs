use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterWebhookReq {
    pub name: String,
    pub url: String,
    pub secret: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterWebhookRes {
    pub id: Uuid,
}

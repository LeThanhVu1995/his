use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Webhook {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub secret: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

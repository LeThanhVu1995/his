use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Message {
    pub id: Uuid,
    pub template_code: Option<String>,
    pub channel: String,
    pub to_addr: String,
    pub subject: Option<String>,
    pub body: String,
    pub status: String,
    pub err: Option<String>,
    pub created_at: DateTime<Utc>,
    pub sent_at: Option<DateTime<Utc>>,
}

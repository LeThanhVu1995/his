use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuditEvent {
    pub id: Uuid,
    pub occurred_at: DateTime<Utc>,
    pub actor_id: Option<Uuid>,
    pub actor_name: Option<String>,
    pub actor_role: Option<String>,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub action: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub tenant_id: Option<Uuid>,
    pub request_id: Option<Uuid>,
    pub source: String,
    pub data: Option<serde_json::Value>,
    pub hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

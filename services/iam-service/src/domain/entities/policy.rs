use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: Uuid,
    pub code: String,
    pub description: Option<String>,
    pub effect: String,
    pub actions: Vec<String>,
    pub resources: Vec<String>,
    pub condition: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

// iam-service src/domain/entities/policy.rs placeholder

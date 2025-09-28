use serde::Deserialize;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize)]
pub struct WriteAuditReq {
    pub event_time: Option<DateTime<Utc>>,
    pub user_id: Option<String>,
    pub entity_name: String,
    pub entity_id: String,
    pub action: String,  // CREATE, UPDATE, DELETE
    pub before_json: Option<String>,
    pub after_json: Option<String>,
    pub ip_address: Option<String>,
}



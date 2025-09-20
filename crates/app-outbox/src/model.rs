// src/model.rs placeholder
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Bản ghi outbox đã lưu trong DB.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboxMsg {
    pub id: Uuid,
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub event_type: String,
    pub topic: String,
    pub partition_key: Option<String>,
    pub headers: Value,
    pub payload: Value,
    pub attempts: i32,
    pub error: Option<String>,
    pub available_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub locked_at: Option<DateTime<Utc>>,
    pub locked_by: Option<String>,
}

impl OutboxMsg {
    /// Serialize payload/headers sang bytes (UTF-8 JSON)
    pub fn payload_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(&self.payload).unwrap_or_default()
    }

    /// Partition key nếu có (ưu tiên trường `partition_key`, fallback aggregate_id)
    pub fn effective_key(&self) -> Option<&str> {
        self.partition_key
            .as_deref()
            .or(Some(self.aggregate_id.as_str()))
    }
}

/// Input để enqueue outbox trong transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewOutboxMsg {
    pub aggregate_type: String,
    pub aggregate_id: String,
    pub event_type: String,
    pub topic: String,
    pub partition_key: Option<String>,
    pub headers: Value,
    pub payload: Value,
    /// Khi nào message sẵn sàng publish (mặc định `now()`)
    pub available_at: Option<DateTime<Utc>>,
}

impl NewOutboxMsg {
    pub fn new<T: Into<String>>(
        aggregate_type: T,
        aggregate_id: T,
        event_type: T,
        topic: T,
        payload: Value,
    ) -> Self {
        Self {
            aggregate_type: aggregate_type.into(),
            aggregate_id: aggregate_id.into(),
            event_type: event_type.into(),
            topic: topic.into(),
            partition_key: None,
            headers: Value::Object(Default::default()),
            payload,
            available_at: None,
        }
    }
}

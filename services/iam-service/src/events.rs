//! IAM domain events (payloads only). Emitted via Outbox → Kafka.
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreated { pub id: Uuid, pub username: String, pub full_name: Option<String>, pub email: Option<String> }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdated { pub id: Uuid, pub username: String, pub full_name: Option<String>, pub email: Option<String>, pub locked: bool }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLocked { pub id: Uuid, pub locked: bool }

pub const IAM_USER_CREATED: &str = "iam.user.created";
pub const IAM_USER_UPDATED: &str = "iam.user.updated";
pub const IAM_USER_LOCKED:  &str = "iam.user.locked";



use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct Page { pub page: Option<i64>, pub page_size: Option<i64> }

#[derive(Debug, Deserialize)]
pub struct ActorQuery { pub actor_id: Uuid, pub page: Option<i64>, pub page_size: Option<i64> }

#[derive(Debug, Deserialize)]
pub struct EntityQuery { pub entity_type: String, pub entity_id: Uuid, pub page: Option<i64>, pub page_size: Option<i64> }



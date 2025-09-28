use serde::{Serialize,Deserialize};
use uuid::Uuid;
use chrono::{DateTime,Utc};

#[derive(Debug,Serialize,Deserialize,sqlx::FromRow)]
pub struct DmsObject{
  pub id:Uuid,
  pub bucket:String,
  pub object_key:String,
  pub content_type:Option<String>,
  pub size:Option<i64>,
  pub sha256:Option<String>,
  pub name:Option<String>,
  pub category:Option<String>,
  pub entity_type:Option<String>,
  pub entity_id:Option<Uuid>,
  pub created_by:Option<Uuid>,
  pub created_at:DateTime<Utc>,
  pub updated_at:DateTime<Utc>
}

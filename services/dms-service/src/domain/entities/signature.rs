use serde::{Serialize,Deserialize};
use uuid::Uuid;
use chrono::{DateTime,Utc};

#[derive(Debug,Serialize,Deserialize,sqlx::FromRow)]
pub struct DmsSignature{
  pub id:Uuid,
  pub object_id:Uuid,
  pub signer_id:Option<Uuid>,
  pub signer_name:Option<String>,
  pub signature_alg:String,
  pub signature_b64:String,
  pub signed_at:DateTime<Utc>,
  pub note:Option<String>
}

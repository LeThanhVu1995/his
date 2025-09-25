use crate::infra::db::repositories::{object_repo::ObjectRepo, signature_repo::SignatureRepo}; use crate::domain::entities::signature::DmsSignature; use uuid::Uuid;
pub struct SignSvc<'a>{ pub db:&'a sqlx::Pool<sqlx::Postgres> }
impl<'a> SignSvc<'a>{
  pub async fn attach_signature(&self, object_id:Uuid, signer_id:Option<Uuid>, signer_name:Option<String>, alg:&str, sig_b64:&str, note:Option<String>) -> anyhow::Result<Uuid>{ let s=DmsSignature{ id:Uuid::new_v4(), object_id, signer_id, signer_name, signature_alg:alg.to_string(), signature_b64:sig_b64.to_string(), signed_at:chrono::Utc::now(), note }; SignatureRepo{db:self.db}.add(&s).await?; Ok(s.id) }
}

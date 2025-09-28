use crate::domain::entities::signature::DmsSignature;
use uuid::Uuid;

pub struct SignSvc<'a>{
  pub db:&'a sqlx::Pool<sqlx::Postgres>
}

impl<'a> SignSvc<'a>{
  pub async fn attach_signature(
    &self,
    object_id:Uuid,
    signer_id:Option<Uuid>,
    signer_name:Option<String>,
    alg:&str,
    sig_b64:&str,
    note:Option<String>
  ) -> anyhow::Result<Uuid>{
    let s=DmsSignature{
      id:Uuid::new_v4(),
      object_id,
      signer_id,
      signer_name,
      signature_alg:alg.to_string(),
      signature_b64:sig_b64.to_string(),
      signed_at:chrono::Utc::now(),
      note
    };
    sqlx::query(
      "INSERT INTO dms_signatures(id,object_id,signer_id,signer_name,signature_alg,signature_b64,note) VALUES($1,$2,$3,$4,$5,$6,$7)"
    )
    .bind(s.id)
    .bind(s.object_id)
    .bind(s.signer_id)
    .bind(s.signer_name)
    .bind(s.signature_alg)
    .bind(s.signature_b64)
    .bind(s.note)
    .execute(self.db).await?;
    Ok(s.id)
  }
}

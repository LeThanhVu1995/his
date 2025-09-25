use uuid::Uuid; use sqlx::{Pool,Postgres}; use crate::domain::entities::signature::DmsSignature;
pub struct SignatureRepo<'a>{ pub db:&'a Pool<Postgres> }
impl<'a> SignatureRepo<'a>{
  pub async fn add(&self, s:&DmsSignature)->anyhow::Result<()> { sqlx::query("INSERT INTO dms_signatures(id,object_id,signer_id,signer_name,signature_alg,signature_b64,note) VALUES($1,$2,$3,$4,$5,$6,$7)").bind(s.id).bind(s.object_id).bind(&s.signer_id).bind(&s.signer_name).bind(&s.signature_alg).bind(&s.signature_b64).bind(&s.note).execute(self.db).await?; Ok(()) }
  pub async fn list_by_object(&self, oid:Uuid)->anyhow::Result<Vec<DmsSignature>>{ Ok(sqlx::query_as::<_,DmsSignature>(r#"SELECT id,object_id,signer_id,signer_name,signature_alg,signature_b64,signed_at,note FROM dms_signatures WHERE object_id=$1 ORDER BY signed_at DESC"#).bind(oid).fetch_all(self.db).await?) }
}

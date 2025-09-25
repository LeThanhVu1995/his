use uuid::Uuid; use sqlx::{Pool,Postgres}; use crate::domain::entities::crossmatch::Crossmatch;
pub struct CrossmatchRepo<'a>{ pub db:&'a Pool<Postgres> }
impl<'a> CrossmatchRepo<'a>{
  pub async fn insert(&self, c:&Crossmatch)->anyhow::Result<()> { sqlx::query("INSERT INTO blood_crossmatches(id,request_id,unit_barcode,abo,rh,result,performed_by,note) VALUES($1,$2,$3,$4,$5,$6,$7,$8)").bind(c.id).bind(c.request_id).bind(&c.unit_barcode).bind(&c.abo).bind(&c.rh).bind(&c.result).bind(c.performed_by).bind(&c.note).execute(self.db).await?; Ok(()) }
  pub async fn list_by_request(&self, req:Uuid)->anyhow::Result<Vec<Crossmatch>>{ Ok(sqlx::query_as::<_,Crossmatch>(r#"SELECT id,request_id,unit_barcode,abo,rh,result,performed_by,performed_at,note FROM blood_crossmatches WHERE request_id=$1 ORDER BY performed_at DESC"#).bind(req).fetch_all(self.db).await?) }
}

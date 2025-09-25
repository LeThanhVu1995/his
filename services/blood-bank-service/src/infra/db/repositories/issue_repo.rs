use uuid::Uuid; use sqlx::{Pool,Postgres}; use crate::domain::entities::issue::Issue;
pub struct IssueRepo<'a>{ pub db:&'a Pool<Postgres> }
impl<'a> IssueRepo<'a>{
  pub async fn insert(&self, i:&Issue)->anyhow::Result<()> { sqlx::query("INSERT INTO blood_issues(id,request_id,unit_barcode,status,released_by,note) VALUES($1,$2,$3,$4,$5,$6)").bind(i.id).bind(i.request_id).bind(&i.unit_barcode).bind(&i.status).bind(i.released_by).bind(&i.note).execute(self.db).await?; Ok(()) }
  pub async fn list_by_request(&self, req:Uuid)->anyhow::Result<Vec<Issue>>{ Ok(sqlx::query_as::<_,Issue>(r#"SELECT id,request_id,unit_barcode,status,released_by,released_at,note FROM blood_issues WHERE request_id=$1 ORDER BY released_at DESC"#).bind(req).fetch_all(self.db).await?) }
}

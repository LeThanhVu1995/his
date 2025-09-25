use uuid::Uuid; use sqlx::{Pool,Postgres}; use crate::domain::entities::cssd_tray::{CssdTray,CssdIssue};
pub struct CssdRepo<'a>{ pub db:&'a Pool<Postgres> }
impl<'a> CssdRepo<'a>{
  pub async fn upsert_tray(&self, code:&str, name:&str)->anyhow::Result<CssdTray>{ let r=sqlx::query_as::<_,CssdTray>(r#"INSERT INTO cssd_trays(id,tray_code,name) VALUES($1,$2,$3) ON CONFLICT(tray_code) DO UPDATE SET name=EXCLUDED.name RETURNING id,tray_code,name,status as "status!",last_cycle_at,last_sterilizer,last_params,created_at,updated_at"#).bind(uuid::Uuid::new_v4()).bind(code).bind(name).fetch_one(self.db).await?; Ok(r) }
  pub async fn set_tray_state(&self, id:Uuid, status:&str, sterilizer:Option<&str>, params:Option<&serde_json::Value>)->anyhow::Result<()> { sqlx::query("UPDATE cssd_trays SET status=$2, last_cycle_at=NOW(), last_sterilizer=$3, last_params=$4, updated_at=NOW() WHERE id=$1").bind(id).bind(status).bind(sterilizer).bind(params).execute(self.db).await?; Ok(()) }
  pub async fn get_tray_by_code(&self, code:&str)->anyhow::Result<Option<CssdTray>>{ Ok(sqlx::query_as::<_,CssdTray>(r#"SELECT id,tray_code,name,status as "status!",last_cycle_at,last_sterilizer,last_params,created_at,updated_at FROM cssd_trays WHERE tray_code=$1"#).bind(code).fetch_optional(self.db).await?) }
  pub async fn issue_tray(&self, tray_id:Uuid, schedule_id:Option<Uuid>, note:Option<&str>)->anyhow::Result<Uuid>{ let id=uuid::Uuid::new_v4(); sqlx::query("INSERT INTO cssd_issues(id,tray_id,schedule_id,note) VALUES($1,$2,$3,$4)").bind(id).bind(tray_id).bind(schedule_id).bind(note).execute(self.db).await?; Ok(id) }
  pub async fn return_tray(&self, issue_id:Uuid)->anyhow::Result<()> { sqlx::query("UPDATE cssd_issues SET returned_at=NOW() WHERE id=$1").bind(issue_id).execute(self.db).await?; Ok(()) }
}

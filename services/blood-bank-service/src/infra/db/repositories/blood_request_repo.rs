use uuid::Uuid; use sqlx::{Pool,Postgres}; use crate::domain::entities::blood_request::BloodRequest;
pub struct BloodRequestRepo<'a>{ pub db:&'a Pool<Postgres> }
impl<'a> BloodRequestRepo<'a>{
  pub async fn insert(&self, r:&BloodRequest)->anyhow::Result<()> { sqlx::query("INSERT INTO blood_requests(id,patient_id,encounter_id,ordering_provider,priority,indication,group_needed,rh_needed,units_requested,status) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)").bind(r.id).bind(r.patient_id).bind(r.encounter_id).bind(r.ordering_provider).bind(&r.priority).bind(&r.indication).bind(&r.group_needed).bind(&r.rh_needed).bind(r.units_requested).bind(&r.status).execute(self.db).await?; Ok(()) }
  pub async fn get(&self, id:Uuid)->anyhow::Result<Option<BloodRequest>>{ Ok(sqlx::query_as::<_,BloodRequest>(r#"SELECT id,patient_id,encounter_id,ordering_provider,priority,indication,group_needed,rh_needed,units_requested,status,created_at,updated_at FROM blood_requests WHERE id=$1"#).bind(id).fetch_optional(self.db).await?) }
  pub async fn update_status(&self, id:Uuid, status:&str)->anyhow::Result<()> { sqlx::query("UPDATE blood_requests SET status=$2, updated_at=NOW() WHERE id=$1").bind(id).bind(status).execute(self.db).await?; Ok(()) }
}

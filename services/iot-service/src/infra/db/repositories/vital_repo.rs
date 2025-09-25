use uuid::Uuid; use sqlx::{Pool,Postgres}; use crate::domain::entities::vital::Vital;
pub struct VitalRepo<'a>{ pub db:&'a Pool<Postgres> }
impl<'a> VitalRepo<'a>{
  pub async fn insert(&self, v:&Vital)->anyhow::Result<()> { sqlx::query("INSERT INTO iot_vitals(id,device_id,patient_id,encounter_id,ts,spo2,hr,sys,dia,map,rr,temp,weight,height,raw) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15)")
    .bind(v.id).bind(v.device_id).bind(v.patient_id).bind(v.encounter_id).bind(v.ts).bind(v.spo2).bind(v.hr).bind(v.sys).bind(v.dia).bind(v.map).bind(v.rr).bind(v.temp).bind(v.weight).bind(v.height).bind(&v.raw)
    .execute(self.db).await?; Ok(()) }
}
// iot-service src/infra/db/repositories/vital_repo.rs placeholder

use uuid::Uuid; use chrono::{DateTime,Utc}; use serde_json::Value as Json; use crate::infra::db::repositories::{device_repo::DeviceRepo, vital_repo::VitalRepo}; use crate::domain::entities::vital::Vital;

pub struct IngestSvc<'a>{ pub db:&'a sqlx::Pool<sqlx::Postgres> }
impl<'a> IngestSvc<'a>{
  pub async fn upsert_device(&self, code:&str, name:&str, typ:&str, location:Option<&str>)->anyhow::Result<uuid::Uuid>{ let dev=DeviceRepo{db:self.db}.upsert(code,name,typ,location).await?; DeviceRepo{db:self.db}.touch_seen(dev.id).await?; Ok(dev.id) }

  pub async fn ingest_vital_json(&self, device_code:&str, payload:&Json)->anyhow::Result<Uuid>{
    let dev=DeviceRepo{db:self.db}.get_by_code(device_code).await?.ok_or(anyhow::anyhow!("device not found"))?;
    let ts=payload.get("ts").and_then(|v|v.as_str()).and_then(|s|s.parse::<DateTime<Utc>>().ok()).unwrap_or_else(|| Utc::now());
    let v=Vital{ id:Uuid::new_v4(), device_id:dev.id,
      patient_id:payload.get("patient_id").and_then(|v|v.as_str()).and_then(|s|Uuid::parse_str(s).ok()),
      encounter_id:payload.get("encounter_id").and_then(|v|v.as_str()).and_then(|s|Uuid::parse_str(s).ok()),
      ts,
      spo2:payload.get("spo2").and_then(|v|v.as_i64()).map(|x|x as i16),
      hr:payload.get("hr").and_then(|v|v.as_i64()).map(|x|x as i16),
      sys:payload.get("sys").and_then(|v|v.as_i64()).map(|x|x as i16),
      dia:payload.get("dia").and_then(|v|v.as_i64()).map(|x|x as i16),
      map:payload.get("map").and_then(|v|v.as_i64()).map(|x|x as i16),
      rr:payload.get("rr").and_then(|v|v.as_i64()).map(|x|x as i16),
      temp:payload.get("temp").and_then(|v|v.as_f64()).map(|x|x as f32),
      weight:payload.get("weight").and_then(|v|v.as_f64()).map(|x|x as f32),
      height:payload.get("height").and_then(|v|v.as_f64()).map(|x|x as f32),
      raw:Some(payload.clone()),
      created_at:Utc::now() };
    VitalRepo{db:self.db}.insert(&v).await?;
    Ok(v.id)
  }
}

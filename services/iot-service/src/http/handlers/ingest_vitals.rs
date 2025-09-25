use actix_web::{post, web, HttpResponse}; use crate::domain::services::ingest_svc::IngestSvc;
#[post("/api/v1/iot/vitals:ingest")] pub async fn ingest(db:web::Data<sqlx::Pool<sqlx::Postgres>>, body:web::Json<crate::http::dto::vital_ingest_dto::VitalIngestReq>) -> actix_web::Result<HttpResponse>{ let id=IngestSvc{db:&db}.ingest_vital_json(&body.device_code, &body.payload).await.map_err(|_|actix_web::error::ErrorBadRequest("payload"))?; Ok(HttpResponse::Created().json(serde_json::json!({"id":id}))) }
// iot-service src/http/handlers/ingest_vitals.rs placeholder

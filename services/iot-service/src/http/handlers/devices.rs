use actix_web::{post, web, HttpResponse}; use crate::domain::services::ingest_svc::IngestSvc;
#[post("/api/v1/iot/devices:upsert")] pub async fn upsert(db:web::Data<sqlx::Pool<sqlx::Postgres>>, body:web::Json<crate::http::dto::device_dto::UpsertDeviceReq>) -> actix_web::Result<HttpResponse>{ let id=IngestSvc{db:&db}.upsert_device(&body.code, &body.name, &body.r#type, body.location.as_deref()).await.map_err(|_|actix_web::error::ErrorInternalServerError("dev"))?; Ok(HttpResponse::Created().json(serde_json::json!({"id":id}))) }
// iot-service src/http/handlers/devices.rs placeholder

use actix_web::{web, HttpResponse, Result};
use actix_web::web::Json;
use crate::domain::services::ingest_svc::IngestSvc;

#[utoipa::path(
    post,
    path = "/api/v1/iot/vitals:ingest",
    request_body = crate::http::dto::vital_ingest_dto::IngestVitalRequest,
    responses(
        (status = 201, description = "Vital data ingested successfully"),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn ingest(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: Json<crate::http::dto::vital_ingest_dto::IngestVitalRequest>,
) -> Result<HttpResponse> {
    let id = IngestSvc { db: &db }
        .ingest_vital_json(&body.device_code, &body.payload)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to ingest vital data"))?;

    Ok(HttpResponse::Created().json(serde_json::json!({"id": id})))
}

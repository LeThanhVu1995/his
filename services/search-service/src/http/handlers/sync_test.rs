use actix_web::{post, web, HttpResponse};
use crate::infra::kafka::consumer::handle_database_event;
use crate::infra::opensearch::client::OsClient;
use serde_json::Value as Json;

#[derive(Debug, serde::Deserialize)]
pub struct SyncTestReq {
    pub event_type: String,
    pub entity_type: String,
    pub entity_id: String,
    pub payload: Json,
}

#[post("/api/v1/search:sync-test")]
pub async fn sync_test(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<SyncTestReq>,
) -> actix_web::Result<HttpResponse> {
    let os = OsClient::from_env();

    match handle_database_event(
        &body.event_type,
        &body.entity_type,
        &body.entity_id,
        &body.payload,
        &db,
        &os,
    ).await {
        Ok(_) => {
            tracing::info!(
                "Successfully processed {} event for {}: {}",
                body.event_type,
                body.entity_type,
                body.entity_id
            );
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": format!("Processed {} event for {}: {}",
                    body.event_type, body.entity_type, body.entity_id)
            })))
        },
        Err(e) => {
            tracing::error!(
                "Failed to process {} event for {}: {} - Error: {}",
                body.event_type,
                body.entity_type,
                body.entity_id,
                e
            );
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": format!("Failed to process event: {}", e)
            })))
        }
    }
}

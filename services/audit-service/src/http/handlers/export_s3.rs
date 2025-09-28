use actix_web::{web, HttpResponse};
use sqlx::Row;
use crate::infra::exporters::s3::S3Exporter;
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client as S3Client;
use std::env;

pub async fn export_to_s3(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    _q: web::Query<serde_json::Value>,
) -> actix_web::Result<HttpResponse> {
    // Get S3 configuration from environment
    let bucket = env::var("S3_BUCKET").unwrap_or_else(|_| "audit-exports".to_string());

    // Create S3 client
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let s3_client = S3Client::new(&config);
    let exporter = S3Exporter::new(s3_client, bucket);

    // Query audit events
    let rows = sqlx::query(
        r#"SELECT to_jsonb(t) AS j FROM (
           SELECT audit_id, event_time, user_id, entity_name, entity_id, action, before_json, after_json, ip_address
           FROM audit_log ORDER BY event_time DESC LIMIT 10000) t"#
    )
    .fetch_all(&**db)
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("db"))?;

    let mut events = Vec::new();
    for r in rows {
        let v: Result<serde_json::Value, _> = r.try_get(0);
        if let Ok(j) = v {
            events.push(j);
        }
    }

    let count = events.len();

    // Export to S3
    match exporter.export_audit_events(&events).await {
        Ok(s3_path) => {
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "message": "Audit events exported to S3",
                "s3_path": s3_path,
                "count": count
            })))
        }
        Err(e) => {
            tracing::error!(?e, "Failed to export to S3");
            Err(actix_web::error::ErrorInternalServerError("S3 export failed"))
        }
    }
}

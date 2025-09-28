use actix_web::{HttpResponse, web};
use sqlx::{Pool, Postgres};
use crate::config::Settings;

static mut PERMISSIONS_REGISTERED: bool = false;

#[utoipa::path(
    get,
    path = "/healthz",
    responses(
        (status = 200, description = "Service is healthy"),
        (status = 503, description = "Service is unhealthy")
    )
)]
pub async fn healthz(
    db: web::Data<Pool<Postgres>>,
    settings: web::Data<Settings>,
) -> actix_web::Result<HttpResponse> {
    // Register permissions with IAM service (only once)
    unsafe {
        if !PERMISSIONS_REGISTERED {
            if let Some(iam_url) = &settings.iam_service_base_url {
                if let Some(token) = &settings.iam_service_token {
                    let permissions = crate::security::policy::permission_catalog("ris-pacs-service");
                    // TODO: Register permissions with IAM service
                    tracing::info!("Registering {} permissions with IAM service", permissions.len());
                }
            }
            PERMISSIONS_REGISTERED = true;
        }
    }

    // Check database connection
    match sqlx::query("SELECT 1").fetch_one(&**db).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "healthy",
            "service": "ris-pacs-service",
            "database": "connected"
        }))),
        Err(e) => {
            tracing::error!("Database health check failed: {}", e);
            Ok(HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "status": "unhealthy",
                "service": "ris-pacs-service",
                "database": "disconnected",
                "error": e.to_string()
            })))
        }
    }
}
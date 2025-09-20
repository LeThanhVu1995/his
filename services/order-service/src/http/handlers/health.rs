use actix_web::{web, HttpResponse, Responder};
use sqlx::Pool;
use sqlx::Postgres;
use std::sync::atomic::{AtomicBool, Ordering};

// Global state để track permission registration status
static PERMISSIONS_REGISTERED: AtomicBool = AtomicBool::new(false);

pub fn set_permissions_registered(registered: bool) {
    PERMISSIONS_REGISTERED.store(registered, Ordering::Relaxed);
}

pub async fn healthz(db: web::Data<Pool<Postgres>>) -> impl Responder {
    // Check database connection
    let db_health = match sqlx::query("SELECT 1").fetch_one(&**db).await {
        Ok(_) => "healthy",
        Err(e) => {
            tracing::error!(error=?e, "Database health check failed");
            "unhealthy"
        }
    };

    // Check permission registration status
    let permissions_health = if PERMISSIONS_REGISTERED.load(Ordering::Relaxed) {
        "registered"
    } else {
        "pending"
    };

    let status = if db_health == "healthy" { "ok" } else { "error" };

    HttpResponse::Ok().json(serde_json::json!({
        "status": status,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "services": {
            "database": db_health,
            "permissions": permissions_health,
            "kafka": "not_checked" // TODO: Add Kafka health check
        },
        "version": env!("CARGO_PKG_VERSION"),
        "service": "order-service"
    }))
}

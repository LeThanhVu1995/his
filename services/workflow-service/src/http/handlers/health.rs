use actix_web::{web, HttpResponse};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

static PERMISSIONS_REGISTERED: AtomicBool = AtomicBool::new(false);

pub fn set_permissions_registered(registered: bool) {
    PERMISSIONS_REGISTERED.store(registered, Ordering::Relaxed);
}

pub async fn healthz(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
) -> actix_web::Result<HttpResponse> {
    // Check database connectivity
    let db_healthy = sqlx::query("SELECT 1")
        .execute(&**db)
        .await
        .is_ok();

    let iam_registered = PERMISSIONS_REGISTERED.load(Ordering::Relaxed);

    let status = if db_healthy && iam_registered {
        "healthy"
    } else {
        "unhealthy"
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": status,
        "checks": {
            "database": if db_healthy { "ok" } else { "error" },
            "iam_permissions": if iam_registered { "registered" } else { "not_registered" }
        }
    })))
}

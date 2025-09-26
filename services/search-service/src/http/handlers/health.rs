use actix_web::{HttpResponse, web};
use sqlx::{Pool, Postgres};

static PERMISSIONS_REGISTERED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

pub fn set_permissions_registered(registered: bool) {
    PERMISSIONS_REGISTERED.store(registered, std::sync::atomic::Ordering::Relaxed);
}

pub async fn healthz(db: web::Data<Pool<Postgres>>) -> actix_web::Result<HttpResponse> {
    let mut healthy = true;
    let mut database_status = "ok".to_string();

    if let Err(e) = db.acquire().await {
        tracing::error!("Database connection error: {:?}", e);
        database_status = format!("error: {}", e);
        healthy = false;
    }

    let permissions_status = if PERMISSIONS_REGISTERED.load(std::sync::atomic::Ordering::Relaxed) { "registered" } else { "pending" };

    let response = serde_json::json!({
        "status": if healthy { "ok" } else { "error" },
        "service": "search-service",
        "database": database_status,
        "permissions": permissions_status,
    });

    if healthy { Ok(HttpResponse::Ok().json(response)) } else { Ok(HttpResponse::InternalServerError().json(response)) }
}

// permissions

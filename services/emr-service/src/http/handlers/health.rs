use actix_web::{web, HttpResponse, Result};
use std::sync::atomic::{AtomicBool, Ordering};

static PERMISSIONS_REGISTERED: AtomicBool = AtomicBool::new(false);

pub async fn healthz() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "emr-service",
        "timestamp": chrono::Utc::now()
    })))
}

pub fn set_permissions_registered(_registered: bool) {
    PERMISSIONS_REGISTERED.store(true, Ordering::SeqCst);
}

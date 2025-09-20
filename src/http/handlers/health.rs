use actix_web::{get, Responder, HttpResponse};
use std::sync::atomic::{AtomicBool, Ordering};

static PERMISSIONS_REGISTERED: AtomicBool = AtomicBool::new(false);

#[get("/healthz")]
pub async fn healthz() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "permissions_registered": PERMISSIONS_REGISTERED.load(Ordering::Relaxed)
    }))
}

pub fn set_permissions_registered() {
    PERMISSIONS_REGISTERED.store(true, Ordering::Relaxed);
}

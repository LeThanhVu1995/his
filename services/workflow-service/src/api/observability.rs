use actix_web::HttpResponse;
pub async fn health() -> impl actix_web::Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

pub async fn metrics() -> impl actix_web::Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "active_instances": 0,
        "pending_tasks": 0,
        "completed_today": 0
    }))
}

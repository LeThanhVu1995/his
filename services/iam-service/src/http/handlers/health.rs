// health handler
use actix_web::{HttpResponse, Responder};

#[utoipa::path(get, path = "/health", tag = "iam", responses((status = 200, description = "Health check")))]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok"
    }))
}

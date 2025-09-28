use actix_web::{HttpResponse, Result};

#[utoipa::path(
    get,
    path = "/healthz",
    responses(
        (status = 200, description = "Service is healthy")
    )
)]
pub async fn healthz() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "iot-service"
    })))
}

pub fn set_permissions_registered(_registered: bool) {
    // Placeholder for permissions registration
}

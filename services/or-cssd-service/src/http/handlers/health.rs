use actix_web::{web, HttpResponse, Result};

pub fn set_permissions_registered(_registered: bool) {}

pub async fn healthz() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

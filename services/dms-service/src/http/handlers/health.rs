use actix_web::{get, HttpResponse};

pub fn set_permissions_registered() {}

#[get("/healthz")]
pub async fn healthz() -> HttpResponse {
    HttpResponse::Ok().finish()
}

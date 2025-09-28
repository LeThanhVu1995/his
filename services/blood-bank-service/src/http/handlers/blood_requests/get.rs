use actix_web::{get, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::http::dto::blood_request_dto::GetBloodRequestResponse;

#[get("/api/v1/blood/requests/{request_id}")]
pub async fn get_blood_request(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let request_id = path.into_inner();
    let blood_request_repo = crate::infra::db::repositories::blood_request_repo::BloodRequestRepo { db: &db };

    match blood_request_repo.get(request_id).await {
        Ok(Some(request)) => {
            HttpResponse::Ok().json(GetBloodRequestResponse { request })
        }
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Blood request not found"
            }))
        }
        Err(e) => {
            tracing::error!("Failed to get blood request: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get blood request",
                "details": e.to_string()
            }))
        }
    }
}

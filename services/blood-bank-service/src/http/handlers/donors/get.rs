use actix_web::{get, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::http::dto::donor_dto::GetDonorResponse;

#[get("/api/v1/blood/donors/{donor_id}")]
pub async fn get_donor(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let donor_id = path.into_inner();
    let donor_repo = crate::infra::db::repositories::donor_repo::DonorRepo { db: &db };
    let donor_svc = crate::domain::services::donor_svc::DonorService { donor_repo };

    match donor_svc.get_donor(donor_id).await {
        Ok(Some(donor)) => {
            HttpResponse::Ok().json(GetDonorResponse { donor })
        }
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Donor not found"
            }))
        }
        Err(e) => {
            tracing::error!("Failed to get donor: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get donor",
                "details": e.to_string()
            }))
        }
    }
}

use actix_web::{get, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::http::dto::donation_dto::GetDonationResponse;

#[get("/api/v1/blood/donations/{donation_id}")]
pub async fn get_donation(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let donation_id = path.into_inner();
    let donation_repo = crate::infra::db::repositories::donation_repo::DonationRepo { db: &db };
    let blood_unit_repo = crate::infra::db::repositories::blood_unit_repo::BloodUnitRepo { db: &db };
    let donation_svc = crate::domain::services::donation_svc::DonationService {
        donation_repo,
        blood_unit_repo: blood_unit_repo.clone()
    };

    match donation_svc.get_donation(donation_id).await {
        Ok(Some(donation)) => {
            // Get associated blood units
            let units = blood_unit_repo.clone().list_by_donation(donation_id).await.unwrap_or_default();

            HttpResponse::Ok().json(GetDonationResponse {
                donation,
                units,
            })
        }
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Donation not found"
            }))
        }
        Err(e) => {
            tracing::error!("Failed to get donation: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get donation",
                "details": e.to_string()
            }))
        }
    }
}

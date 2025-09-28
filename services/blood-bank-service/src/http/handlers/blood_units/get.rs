use actix_web::{get, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::http::dto::blood_unit_dto::GetUnitResponse;

#[get("/api/v1/blood/units/{unit_id}")]
pub async fn get_unit(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let unit_id = path.into_inner();
    let blood_unit_repo = crate::infra::db::repositories::blood_unit_repo::BloodUnitRepo { db: &db };
    let donation_repo = crate::infra::db::repositories::donation_repo::DonationRepo { db: &db };
    let donor_repo = crate::infra::db::repositories::donor_repo::DonorRepo { db: &db };

    match blood_unit_repo.get_by_id(unit_id).await {
        Ok(Some(unit)) => {
            // Get associated donation and donor
            let donation = donation_repo.get_by_id(unit.donation_id).await.ok().flatten();
            let donor = if let Some(ref donation) = donation {
                donor_repo.get_by_id(donation.donor_id).await.ok().flatten()
            } else {
                None
            };

            HttpResponse::Ok().json(GetUnitResponse {
                unit,
                donation,
                donor,
            })
        }
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Blood unit not found"
            }))
        }
        Err(e) => {
            tracing::error!("Failed to get blood unit: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get blood unit",
                "details": e.to_string()
            }))
        }
    }
}

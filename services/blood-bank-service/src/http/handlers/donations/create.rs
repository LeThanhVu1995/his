use actix_web::{post, web, HttpResponse, Responder};
use crate::http::dto::donation_dto::{CreateDonationRequest, CreateDonationResponse};

#[post("/api/v1/blood/donations")]
pub async fn create_donation(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<CreateDonationRequest>,
) -> impl Responder {
    let donation_repo = crate::infra::db::repositories::donation_repo::DonationRepo { db: &db };
    let blood_unit_repo = crate::infra::db::repositories::blood_unit_repo::BloodUnitRepo { db: &db };
    let donation_svc = crate::domain::services::donation_svc::DonationService {
        donation_repo,
        blood_unit_repo
    };

    match donation_svc.record_donation(
        body.donor_id,
        body.volume_ml,
        body.remarks.clone(),
    ).await {
        Ok(donation_id) => {
            // Process donation into blood units
            match donation_svc.process_donation_into_units(
                donation_id,
                body.component_codes.clone(),
                body.blood_group.clone(),
            ).await {
                Ok(unit_ids) => {
                    HttpResponse::Created().json(CreateDonationResponse {
                        donation_id,
                        unit_ids,
                        message: "Donation recorded and processed successfully".to_string(),
                    })
                }
                Err(e) => {
                    tracing::error!("Failed to process donation into units: {:?}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to process donation into units",
                        "details": e.to_string()
                    }))
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to create donation: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create donation",
                "details": e.to_string()
            }))
        }
    }
}

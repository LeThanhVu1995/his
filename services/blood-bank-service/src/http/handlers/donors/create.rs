use actix_web::{post, web, HttpResponse, Responder};
use crate::http::dto::donor_dto::{CreateDonorRequest, CreateDonorResponse};

#[post("/api/v1/blood/donors")]
pub async fn create_donor(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<CreateDonorRequest>,
) -> impl Responder {
    let donor_repo = crate::infra::db::repositories::donor_repo::DonorRepo { db: &db };
    let donor_svc = crate::domain::services::donor_svc::DonorService { donor_repo };

    match donor_svc.register_donor(
        body.code.clone(),
        body.name.clone(),
        body.date_of_birth,
        body.gender.clone(),
        body.blood_group.clone(),
        body.phone.clone(),
    ).await {
        Ok(donor_id) => {
            HttpResponse::Created().json(CreateDonorResponse {
                donor_id,
                message: "Donor registered successfully".to_string(),
            })
        }
        Err(e) => {
            tracing::error!("Failed to create donor: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create donor",
                "details": e.to_string()
            }))
        }
    }
}

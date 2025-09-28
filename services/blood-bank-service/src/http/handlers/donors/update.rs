use actix_web::{put, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::http::dto::donor_dto::{UpdateDonorRequest, UpdateDonorResponse};

#[put("/api/v1/blood/donors/{donor_id}")]
pub async fn update_donor(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateDonorRequest>,
) -> impl Responder {
    let donor_id = path.into_inner();
    let donor_repo = crate::infra::db::repositories::donor_repo::DonorRepo { db: &db };
    let donor_svc = crate::domain::services::donor_svc::DonorService { donor_repo };

    match donor_svc.update_donor_info(
        donor_id,
        body.name.clone(),
        body.date_of_birth,
        body.gender.clone(),
        body.blood_group.clone(),
        body.phone.clone(),
    ).await {
        Ok(()) => {
            HttpResponse::Ok().json(UpdateDonorResponse {
                message: "Donor updated successfully".to_string(),
            })
        }
        Err(e) => {
            tracing::error!("Failed to update donor: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update donor",
                "details": e.to_string()
            }))
        }
    }
}

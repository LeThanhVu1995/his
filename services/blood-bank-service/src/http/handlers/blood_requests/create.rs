use actix_web::{post, web, HttpResponse, Responder};
use crate::http::dto::blood_request_dto::{CreateBloodRequestRequest, CreateBloodRequestResponse};

#[post("/api/v1/blood/requests")]
pub async fn create_blood_request(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<CreateBloodRequestRequest>,
) -> impl Responder {
    let blood_request_repo = crate::infra::db::repositories::blood_request_repo::BloodRequestRepo { db: &db };

    let request_id = uuid::Uuid::new_v4();
    let now = chrono::Utc::now();

    let blood_request = crate::domain::entities::blood_request::BloodRequest {
        request_id,
        patient_id: body.patient_id,
        encounter_id: body.encounter_id,
        ordering_provider: body.ordering_provider,
        blood_group: body.blood_group.clone(),
        component_code: body.component_code.clone(),
        quantity: body.quantity,
        priority: body.priority.clone(),
        indication: body.indication.clone(),
        status: "PENDING".to_string(),
        requested_by: None, // Will be set by auth user in real implementation
        requested_at: Some(now),
        created_at: now,
        updated_at: now,
    };

    match blood_request_repo.insert(&blood_request).await {
        Ok(()) => {
            HttpResponse::Created().json(CreateBloodRequestResponse {
                request_id,
                message: "Blood request created successfully".to_string(),
            })
        }
        Err(e) => {
            tracing::error!("Failed to create blood request: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create blood request",
                "details": e.to_string()
            }))
        }
    }
}

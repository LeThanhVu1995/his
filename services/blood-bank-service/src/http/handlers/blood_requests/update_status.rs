use actix_web::{put, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::http::dto::blood_request_dto::{UpdateBloodRequestStatusRequest, UpdateBloodRequestStatusResponse};

#[put("/api/v1/blood/requests/{request_id}/status")]
pub async fn update_blood_request_status(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateBloodRequestStatusRequest>,
) -> impl Responder {
    let request_id = path.into_inner();
    let blood_request_repo = crate::infra::db::repositories::blood_request_repo::BloodRequestRepo { db: &db };

    // Validate status
    let valid_statuses = ["PENDING", "FULFILLED", "CANCELLED"];
    if !valid_statuses.contains(&body.status.as_str()) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid status",
            "valid_statuses": valid_statuses
        }));
    }

    match blood_request_repo.update_status(request_id, &body.status).await {
        Ok(()) => {
            tracing::info!("Blood request {} status updated to {}: {}",
                request_id, body.status, body.reason.as_deref().unwrap_or("No reason provided"));

            HttpResponse::Ok().json(UpdateBloodRequestStatusResponse {
                message: format!("Blood request status updated to {}", body.status),
            })
        }
        Err(e) => {
            tracing::error!("Failed to update blood request status: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update blood request status",
                "details": e.to_string()
            }))
        }
    }
}

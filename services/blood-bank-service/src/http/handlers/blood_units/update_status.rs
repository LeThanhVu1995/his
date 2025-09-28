use actix_web::{put, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::http::dto::blood_unit_dto::{UpdateUnitStatusRequest, UpdateUnitStatusResponse};

#[put("/api/v1/blood/units/{unit_id}/status")]
pub async fn update_unit_status(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateUnitStatusRequest>,
) -> impl Responder {
    let unit_id = path.into_inner();
    let blood_unit_repo = crate::infra::db::repositories::blood_unit_repo::BloodUnitRepo { db: &db };
    let blood_unit_svc = crate::domain::services::blood_unit_svc::BloodUnitService { blood_unit_repo };

    // Validate status transition
    let valid_statuses = ["AVAILABLE", "RESERVED", "CROSSMATCHED", "ISSUED", "EXPIRED"];
    if !valid_statuses.contains(&body.status.as_str()) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid status",
            "valid_statuses": valid_statuses
        }));
    }

    match blood_unit_svc.update_status(unit_id, &body.status).await {
        Ok(()) => {
            tracing::info!("Blood unit {} status updated to {}: {}",
                unit_id, body.status, body.reason.as_deref().unwrap_or("No reason provided"));

            HttpResponse::Ok().json(UpdateUnitStatusResponse {
                message: format!("Blood unit status updated to {}", body.status),
            })
        }
        Err(e) => {
            tracing::error!("Failed to update blood unit status: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to update blood unit status",
                "details": e.to_string()
            }))
        }
    }
}

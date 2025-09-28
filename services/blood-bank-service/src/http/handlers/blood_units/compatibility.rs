use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CheckCompatibilityRequest {
    pub patient_blood_group: String,
    pub component_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CheckCompatibilityResponse {
    pub compatible_units: Vec<crate::domain::entities::blood_unit::BloodUnit>,
    pub total_compatible: usize,
}

#[post("/api/v1/blood/units:check-compatibility")]
pub async fn check_compatibility(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<CheckCompatibilityRequest>,
) -> impl Responder {
    let blood_unit_repo = crate::infra::db::repositories::blood_unit_repo::BloodUnitRepo { db: &db };
    let blood_unit_svc = crate::domain::services::blood_unit_svc::BloodUnitService { blood_unit_repo };

    match blood_unit_svc.find_compatible_units(
        &body.patient_blood_group,
        body.component_code.as_deref(),
    ).await {
        Ok(compatible_units) => {
            HttpResponse::Ok().json(CheckCompatibilityResponse {
                total_compatible: compatible_units.len(),
                compatible_units,
            })
        }
        Err(e) => {
            tracing::error!("Failed to check compatibility: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to check compatibility",
                "details": e.to_string()
            }))
        }
    }
}

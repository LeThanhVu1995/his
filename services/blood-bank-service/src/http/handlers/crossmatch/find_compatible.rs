use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct FindCompatibleRequest {
    pub patient_id: Uuid,
    pub blood_group: String,
    pub component_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FindCompatibleResponse {
    pub compatible_unit_ids: Vec<Uuid>,
    pub total_found: usize,
    pub message: String,
}

#[post("/api/v1/blood/crossmatch:find-compatible")]
pub async fn find_compatible_units(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<FindCompatibleRequest>,
) -> impl Responder {
    let crossmatch_repo = crate::infra::db::repositories::crossmatch_repo::CrossmatchRepo { db: &db };
    let blood_unit_repo = crate::infra::db::repositories::blood_unit_repo::BloodUnitRepo { db: &db };
    let crossmatch_svc = crate::domain::services::crossmatch_svc::CrossmatchService {
        crossmatch_repo,
        blood_unit_repo
    };

    match crossmatch_svc.get_compatible_units_for_patient(
        body.patient_id,
        &body.blood_group,
        body.component_code.as_deref(),
    ).await {
        Ok(compatible_unit_ids) => {
            let total_found = compatible_unit_ids.len();
            HttpResponse::Ok().json(FindCompatibleResponse {
                total_found,
                compatible_unit_ids,
                message: format!("Found {} compatible units", total_found),
            })
        }
        Err(e) => {
            tracing::error!("Failed to find compatible units: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to find compatible units",
                "details": e.to_string()
            }))
        }
    }
}

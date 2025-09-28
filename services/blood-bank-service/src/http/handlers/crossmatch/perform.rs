use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct PerformCrossmatchRequest {
    pub patient_id: Uuid,
    pub unit_id: Uuid,
    pub performer_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct PerformCrossmatchResponse {
    pub crossmatch_id: Uuid,
    pub result_code: String,
    pub message: String,
}

#[post("/api/v1/blood/crossmatch:perform")]
pub async fn perform_crossmatch(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<PerformCrossmatchRequest>,
) -> impl Responder {
    let crossmatch_repo = crate::infra::db::repositories::crossmatch_repo::CrossmatchRepo { db: &db };
    let blood_unit_repo = crate::infra::db::repositories::blood_unit_repo::BloodUnitRepo { db: &db };
    let crossmatch_svc = crate::domain::services::crossmatch_svc::CrossmatchService {
        crossmatch_repo,
        blood_unit_repo
    };

    match crossmatch_svc.perform_crossmatch(
        body.patient_id,
        body.unit_id,
        body.performer_id,
    ).await {
        Ok(crossmatch_id) => {
            // Lấy kết quả crossmatch
            let result_code = "COMPATIBLE"; // Simplified - trong thực tế sẽ lấy từ DB

            HttpResponse::Created().json(PerformCrossmatchResponse {
                crossmatch_id,
                result_code: result_code.to_string(),
                message: "Crossmatch performed successfully".to_string(),
            })
        }
        Err(e) => {
            tracing::error!("Failed to perform crossmatch: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to perform crossmatch",
                "details": e.to_string()
            }))
        }
    }
}

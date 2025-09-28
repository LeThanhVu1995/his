use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ReturnBloodRequest {
    pub unit_id: Uuid,
    pub reason: String,
}

#[derive(Debug, Serialize)]
pub struct ReturnBloodResponse {
    pub message: String,
}

#[post("/api/v1/blood/issues:return")]
pub async fn return_blood_unit(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<ReturnBloodRequest>,
) -> impl Responder {
    let issue_repo = crate::infra::db::repositories::issue_repo::IssueRepo { db: &db };
    let blood_unit_repo = crate::infra::db::repositories::blood_unit_repo::BloodUnitRepo { db: &db };
    let crossmatch_repo = crate::infra::db::repositories::crossmatch_repo::CrossmatchRepo { db: &db };
    let issue_svc = crate::domain::services::issue_svc::IssueService {
        issue_repo,
        blood_unit_repo,
        crossmatch_repo
    };

    match issue_svc.return_blood_unit(
        body.unit_id,
        &body.reason,
    ).await {
        Ok(()) => {
            HttpResponse::Ok().json(ReturnBloodResponse {
                message: "Blood unit returned successfully".to_string(),
            })
        }
        Err(e) => {
            tracing::error!("Failed to return blood unit: {:?}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Failed to return blood unit",
                "details": e.to_string()
            }))
        }
    }
}

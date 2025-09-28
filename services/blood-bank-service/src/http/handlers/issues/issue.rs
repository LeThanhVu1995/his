use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct IssueBloodRequest {
    pub unit_id: Uuid,
    pub encounter_id: Uuid,
    pub issued_by: Uuid,
}

#[derive(Debug, Serialize)]
pub struct IssueBloodResponse {
    pub issue_id: Uuid,
    pub message: String,
}

#[post("/api/v1/blood/issues")]
pub async fn issue_blood_unit(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<IssueBloodRequest>,
) -> impl Responder {
    let issue_repo = crate::infra::db::repositories::issue_repo::IssueRepo { db: &db };
    let blood_unit_repo = crate::infra::db::repositories::blood_unit_repo::BloodUnitRepo { db: &db };
    let crossmatch_repo = crate::infra::db::repositories::crossmatch_repo::CrossmatchRepo { db: &db };
    let issue_svc = crate::domain::services::issue_svc::IssueService {
        issue_repo,
        blood_unit_repo,
        crossmatch_repo
    };

    match issue_svc.issue_blood_unit(
        body.unit_id,
        body.encounter_id,
        body.issued_by,
    ).await {
        Ok(issue_id) => {
            HttpResponse::Created().json(IssueBloodResponse {
                issue_id,
                message: "Blood unit issued successfully".to_string(),
            })
        }
        Err(e) => {
            tracing::error!("Failed to issue blood unit: {:?}", e);
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Failed to issue blood unit",
                "details": e.to_string()
            }))
        }
    }
}

use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ListIssuesQuery {
    pub encounter_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct ListIssuesResponse {
    pub issues: Vec<crate::domain::entities::issue::Issue>,
    pub total: usize,
    pub encounter_id: Uuid,
}

#[get("/api/v1/blood/issues")]
pub async fn list_issues(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: web::Query<ListIssuesQuery>,
) -> impl Responder {
    let issue_repo = crate::infra::db::repositories::issue_repo::IssueRepo { db: &db };
    let blood_unit_repo = crate::infra::db::repositories::blood_unit_repo::BloodUnitRepo { db: &db };
    let crossmatch_repo = crate::infra::db::repositories::crossmatch_repo::CrossmatchRepo { db: &db };
    let issue_svc = crate::domain::services::issue_svc::IssueService {
        issue_repo,
        blood_unit_repo,
        crossmatch_repo
    };

    match issue_svc.list_issues_by_encounter(query.encounter_id).await {
        Ok(issues) => {
            HttpResponse::Ok().json(ListIssuesResponse {
                total: issues.len(),
                issues,
                encounter_id: query.encounter_id,
            })
        }
        Err(e) => {
            tracing::error!("Failed to list issues: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to list issues",
                "details": e.to_string()
            }))
        }
    }
}

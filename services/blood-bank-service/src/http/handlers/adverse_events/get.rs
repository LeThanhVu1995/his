use actix_web::{get, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::http::dto::adverse_event_dto::GetAdverseEventResponse;

#[get("/api/v1/blood/adverse-events/{event_id}")]
pub async fn get_adverse_event(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let event_id = path.into_inner();
    let adverse_event_repo = crate::infra::db::repositories::adverse_event_repo::AdverseEventRepo { db: &db };
    let issue_repo = crate::infra::db::repositories::issue_repo::IssueRepo { db: &db };
    let adverse_event_svc = crate::domain::services::adverse_event_svc::AdverseEventService { adverse_event_repo };

    match adverse_event_svc.get_event(event_id).await {
        Ok(Some(event)) => {
            // Get associated issue
            let issue = issue_repo.get_by_id(event.issue_id).await.ok().flatten();

            HttpResponse::Ok().json(GetAdverseEventResponse {
                event,
                issue,
            })
        }
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Adverse event not found"
            }))
        }
        Err(e) => {
            tracing::error!("Failed to get adverse event: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get adverse event",
                "details": e.to_string()
            }))
        }
    }
}

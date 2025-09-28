use actix_web::{get, web, HttpResponse, Responder};
use crate::http::dto::adverse_event_dto::{ListAdverseEventsQuery, ListAdverseEventsResponse};

#[get("/api/v1/blood/adverse-events")]
pub async fn list_adverse_events(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    query: web::Query<ListAdverseEventsQuery>,
) -> impl Responder {
    let adverse_event_repo = crate::infra::db::repositories::adverse_event_repo::AdverseEventRepo { db: &db };
    let adverse_event_svc = crate::domain::services::adverse_event_svc::AdverseEventService { adverse_event_repo };

    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);

    let events = if let Some(issue_id) = query.issue_id {
        adverse_event_svc.list_events_by_issue(issue_id, limit, offset).await
    } else {
        adverse_event_svc.list_recent_events(limit).await
    };

    match events {
        Ok(events) => {
            HttpResponse::Ok().json(ListAdverseEventsResponse {
                total: events.len(),
                events,
                limit,
                offset,
            })
        }
        Err(e) => {
            tracing::error!("Failed to list adverse events: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to list adverse events",
                "details": e.to_string()
            }))
        }
    }
}

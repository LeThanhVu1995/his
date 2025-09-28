use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ReportAdverseEventRequest {
    pub issue_id: Uuid,
    pub type_code: Option<String>,
    pub severity_code: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReportAdverseEventResponse {
    pub event_id: Uuid,
    pub message: String,
}

#[post("/api/v1/blood/adverse-events")]
pub async fn report_adverse_event(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<ReportAdverseEventRequest>,
) -> impl Responder {
    let adverse_event_repo = crate::infra::db::repositories::adverse_event_repo::AdverseEventRepo { db: &db };
    let adverse_event_svc = crate::domain::services::adverse_event_svc::AdverseEventService { adverse_event_repo };

    // Auto-classify severity if not provided
    let severity_code = if body.severity_code.is_some() {
        body.severity_code.clone()
    } else if let Some(description) = &body.description {
        Some(adverse_event_svc.classify_severity(description).await)
    } else {
        Some("UNKNOWN".to_string())
    };

    match adverse_event_svc.report_adverse_event(
        body.issue_id,
        body.type_code.clone(),
        severity_code,
        body.description.clone(),
    ).await {
        Ok(event_id) => {
            HttpResponse::Created().json(ReportAdverseEventResponse {
                event_id,
                message: "Adverse event reported successfully".to_string(),
            })
        }
        Err(e) => {
            tracing::error!("Failed to report adverse event: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to report adverse event",
                "details": e.to_string()
            }))
        }
    }
}

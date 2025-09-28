use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::{infra::db::repositories::audit_repo::AuditRepo, domain::entities::audit_event::AuditEvent};

pub async fn write(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<crate::http::dto::write_dto::WriteAuditReq>,
) -> actix_web::Result<HttpResponse> {
    if std::env::var("ALLOW_DIRECT_WRITE").unwrap_or("false".into()) != "true" {
        return Err(actix_web::error::ErrorForbidden("disabled"));
    }
    let ev = AuditEvent {
        audit_id: Uuid::new_v4().to_string(),
        event_time: body.event_time.unwrap_or(chrono::Utc::now()),
        user_id: body.user_id.clone(),
        entity_name: body.entity_name.clone(),
        entity_id: body.entity_id.clone(),
        action: body.action.clone(),
        before_json: body.before_json.clone(),
        after_json: body.after_json.clone(),
        ip_address: body.ip_address.clone(),
    };
    AuditRepo { db: &db }.insert(&ev)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("db"))?;
    Ok(HttpResponse::Created().json(serde_json::json!({"audit_id": ev.audit_id})))
}



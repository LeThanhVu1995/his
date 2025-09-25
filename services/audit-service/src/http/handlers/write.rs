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
        id: Uuid::new_v4(),
        occurred_at: body.occurred_at.unwrap_or(chrono::Utc::now()),
        actor_id: body.actor_id,
        actor_name: body.actor_name.clone(),
        actor_role: body.actor_role.clone(),
        ip: body.ip.clone(),
        user_agent: body.user_agent.clone(),
        action: body.action.clone(),
        entity_type: body.entity_type.clone(),
        entity_id: body.entity_id,
        tenant_id: body.tenant_id,
        request_id: body.request_id,
        source: body.source.clone(),
        data: body.data.clone(),
        hash: None,
        created_at: chrono::Utc::now(),
    };
    AuditRepo { db: &db }.insert(&ev)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("db"))?;
    Ok(HttpResponse::Created().json(serde_json::json!({"id": ev.id})))
}



use actix_web::{web, HttpResponse};
use crate::infra::db::repositories::audit_repo::AuditRepo;

pub async fn by_entity(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: web::Query<crate::http::dto::query_dto::EntityQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1).max(1);
    let size = q.page_size.unwrap_or(50).clamp(1, 200);
    let off = (page - 1) * size;
    let rows = AuditRepo { db: &db }
        .by_entity(&q.entity_type, q.entity_id, size, off)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("db"))?;
    Ok(HttpResponse::Ok().json(rows))
}

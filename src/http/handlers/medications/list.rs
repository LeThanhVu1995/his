use actix_web::{get, web, HttpResponse};
use uuid::Uuid;
use crate::domain::repo::MedRepo;
use crate::http::dto::medication_dto::{MedQuery, MedicationRes};
use crate::http::dto::common::calc_etag;

#[get("/api/v1/medications")]
pub async fn list_meds(
    req: actix_web::HttpRequest,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: web::Query<MedQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);
    let repo = MedRepo { db: &db };
    let (items, total) = repo
        .search_paged(q.q.as_deref(), page, size)
        .await
        .map_err(|e| {
            tracing::error!(?e, "list meds");
            crate::error::AppError::Internal("DB".into())
        })?;
    let res: Vec<MedicationRes> = items
        .into_iter()
        .map(|m| MedicationRes {
            id: m.id,
            code: m.code,
            name: m.name,
            strength: m.strength,
            form: m.form,
            route: m.route,
        })
        .collect();
    let body = serde_json::to_vec(&res).unwrap();
    let etag = calc_etag(&body);
    if let Some(tag) = req
        .headers()
        .get(actix_web::http::header::IF_NONE_MATCH)
        .and_then(|h| h.to_str().ok())
    {
        if tag == etag {
            return Ok(HttpResponse::NotModified().finish());
        }
    }
    Ok(HttpResponse::Ok()
        .append_header((actix_web::http::header::ETAG, etag))
        .append_header(("X-Total-Count", total.to_string()))
        .append_header(("X-Page", page.to_string()))
        .append_header(("X-Page-Size", size.to_string()))
        .content_type("application/json")
        .body(body))
}

use actix_web::{get, web, HttpResponse};
use uuid::Uuid;
use crate::domain::repo::PrescRepo;
use crate::http::dto::prescription_dto::{PrescQuery, PrescriptionRes};
use crate::http::dto::common::calc_etag;

#[get("/api/v1/prescriptions")]
pub async fn list_prescriptions(
    req: actix_web::HttpRequest,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: web::Query<PrescQuery>,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);
    let repo = PrescRepo { db: &db };
    let (items, total) = repo
        .list_paged(q.patient_id, q.status.as_deref(), page, size)
        .await
        .map_err(|e| {
            tracing::error!(?e, "list presc");
            crate::error::AppError::Internal("DB".into())
        })?;
    let res: Vec<PrescriptionRes> = items
        .into_iter()
        .map(|p| PrescriptionRes {
            id: p.id,
            presc_no: p.presc_no,
            status: p.status,
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

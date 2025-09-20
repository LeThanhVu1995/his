use actix_web::{web, HttpResponse, get};
use actix_web_validator::Query;
use uuid::Uuid;
use app_web::prelude::AuthUser;
use crate::infra::db::repositories::invoice_repo;
use crate::http::dto::invoice_dto::{InvoiceQuery, InvoiceRes};
use billing_service::dto::common::calc_etag;

#[get("/api/v1/invoices")]
#[utoipa::path(
    get,
    path = "/api/v1/invoices",
    params(("encounter_id"=Option<uuid::Uuid>, Query, description="Encounter ID"),("status"=Option<String>, Query, description="Status"),("page"=Option<i64>, Query, description="Page number"),("page_size"=Option<i64>, Query, description="Page size")),
    security(("bearer_auth" = []))
)]
pub async fn list_invoices(
    req: actix_web::HttpRequest,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: Query<InvoiceQuery>,
    _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);

    let (items, total) = invoice_repo::list_paged(
        &db,
        q.encounter_id,
        q.status.as_deref(),
        page,
        size,
    )
    .await
    .map_err(|e| {
        tracing::error!(?e, "list invoices");
        crate::error::AppError::Internal("DB".into())
    })?;

    let res: Vec<InvoiceRes> = items
        .into_iter()
        .map(|i| InvoiceRes {
            id: i.id,
            invoice_no: i.invoice_no,
            total: i.total,
            status: i.status,
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

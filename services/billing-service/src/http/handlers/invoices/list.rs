use actix_web::{web, HttpResponse, get};
use actix_web_validator::Query;
use app_web::prelude::AuthUser;
use crate::http::dto::invoice_dto::{InvoiceQuery, InvoiceRes};
use crate::http::handlers::common::create_billing_service;
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

    let billing_service = create_billing_service(&db);

    let items = if let Some(encounter_id) = q.encounter_id {
        billing_service.list_invoices_by_encounter(encounter_id, size, (page - 1) * size).await
            .map_err(|e| {
                tracing::error!(?e, "list invoices by encounter");
                crate::error::AppError::Internal("DB".into())
            })?
    } else {
        vec![]
    };

    let res: Vec<InvoiceRes> = items
        .into_iter()
        .map(|i| InvoiceRes {
            id: i.invoice_id,
            invoice_no: format!("INV-{}", &i.invoice_id.to_string()[..8]),
            total: i.total_amount,
            status: i.status,
        })
        .collect();

    let body = serde_json::to_vec(&res).unwrap();
    let etag = format!("\"{}\"", body.len());

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
        .append_header(("X-Total-Count", res.len().to_string()))
        .append_header(("X-Page", page.to_string()))
        .append_header(("X-Page-Size", size.to_string()))
        .content_type("application/json")
        .body(body))
}

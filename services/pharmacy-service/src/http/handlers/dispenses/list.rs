use actix_web::{web, HttpResponse};
use actix_web_validator::Query;

use app_web::prelude::AuthUser;
use crate::domain::repositories::DispenseRepo;
use crate::http::dto::dispense_dto::{DispenseQuery, DispenseRes};
use crate::http::dto::pagination::calc_etag;


#[utoipa::path(
    get,
    path = "/api/v1/dispenses",
    params(("prescription_id"=Option<uuid::Uuid>, Query, description="Prescription ID"),("page"=Option<i64>, Query, description="Page number"),("page_size"=Option<i64>, Query, description="Page size")),
    security(("bearer_auth" = []))
)]
pub async fn list_dispenses(
    req: actix_web::HttpRequest,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: Query<DispenseQuery>,
    _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);

    let repo = DispenseRepo { db: &db };
    let (items, total) = repo
        .list_paged(q.prescription_id, page, size)
        .await
    .map_err(|e| {
        tracing::error!(?e, "list dispenses");
        crate::error::AppError::Internal("DB".into())
    })?;

    let res: Vec<DispenseRes> = items
        .into_iter()
        .map(|d| DispenseRes {
            id: d.id,
            prescription_id: d.prescription_id,
            disp_no: d.disp_no,
            dispensed_by: d.dispensed_by,
            status: d.status,
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

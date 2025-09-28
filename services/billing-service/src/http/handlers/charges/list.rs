use actix_web::{web, HttpResponse, get};
use actix_web_validator::Query;
use app_web::prelude::AuthUser;
use crate::http::dto::charge_dto::{ChargeQuery, ChargeRes};
use crate::http::handlers::common::create_billing_service;

#[get("/api/v1/charges")]
#[utoipa::path(
    get,
    path = "/api/v1/charges",
    params(("encounter_id"=Option<uuid::Uuid>, Query, description="Encounter ID"),("status"=Option<String>, Query, description="Status"),("page"=Option<i64>, Query, description="Page number"),("page_size"=Option<i64>, Query, description="Page size")),
    security(("bearer_auth" = []))
)]
pub async fn list_charges(
    req: actix_web::HttpRequest,
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: Query<ChargeQuery>,
    _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);

    let billing_service = create_billing_service(&db);

    let items = if let Some(encounter_id) = q.encounter_id {
        billing_service.list_charges_by_encounter(encounter_id, size, (page - 1) * size).await
            .map_err(|e| {
                tracing::error!(?e, "list charges by encounter");
                crate::error::AppError::Internal("DB".into())
            })?
    } else {
        // For now, return empty list - in real implementation would have a general list method
        vec![]
    };

    let res: Vec<ChargeRes> = items
        .into_iter()
        .map(|c| ChargeRes {
            id: c.charge_id,
            code: c.service_code,
            name: c.description.unwrap_or_default(),
            qty: c.qty,
            unit_price: c.unit_price,
            amount: c.amount,
            status: c.status,
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


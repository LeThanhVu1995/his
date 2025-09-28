use actix_web::{web, HttpResponse, get};
use actix_web_validator::Query;
use app_web::prelude::AuthUser;
use crate::http::dto::refund_dto::{RefundQuery, RefundRes};
use crate::http::handlers::common::create_billing_service;

#[get("/api/v1/refunds")]
#[utoipa::path(
    get,
    path = "/api/v1/refunds",
    params(("payment_id"=Option<uuid::Uuid>, Query, description="Payment ID"),("page"=Option<i64>, Query, description="Page number"),("page_size"=Option<i64>, Query, description="Page size")),
    security(("bearer_auth" = []))
)]
pub async fn list_refunds(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: Query<RefundQuery>,
    _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);

    let billing_service = create_billing_service(&db);

    let items = if let Some(payment_id) = q.payment_id {
        billing_service.list_refunds_by_payment(payment_id).await
            .map_err(|e| {
                tracing::error!(?e, "list refunds by payment");
                crate::error::AppError::Internal("DB".into())
            })?
    } else {
        vec![]
    };

    let res: Vec<RefundRes> = items
        .into_iter()
        .map(|r| RefundRes {
            id: r.refund_id,
            payment_id: r.payment_id,
            amount: r.amount,
            reason: r.reason,
            status: r.status,
        })
        .collect();

    Ok(HttpResponse::Ok()
        .append_header(("X-Total-Count", res.len().to_string()))
        .append_header(("X-Page", page.to_string()))
        .append_header(("X-Page-Size", size.to_string()))
        .json(res))
}

use actix_web::{web, HttpResponse, get};
use actix_web_validator::Query;
use app_web::prelude::AuthUser;
use crate::http::dto::payment_dto::{PaymentQuery, PaymentRes};
use crate::http::handlers::common::create_billing_service;

#[get("/api/v1/payments")]
#[utoipa::path(
    get,
    path = "/api/v1/payments",
    params(("invoice_id"=Option<uuid::Uuid>, Query, description="Invoice ID"),("page"=Option<i64>, Query, description="Page number"),("page_size"=Option<i64>, Query, description="Page size")),
    security(("bearer_auth" = []))
)]
pub async fn list_payments(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    q: Query<PaymentQuery>,
    _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let page = q.page.unwrap_or(1);
    let size = q.page_size.unwrap_or(50);

    let billing_service = create_billing_service(&db);

    let items = if let Some(invoice_id) = q.invoice_id {
        billing_service.list_payments_by_invoice(invoice_id).await
            .map_err(|e| {
                tracing::error!(?e, "list payments by invoice");
                crate::error::AppError::Internal("DB".into())
            })?
    } else {
        vec![]
    };

    let res: Vec<PaymentRes> = items
        .into_iter()
        .map(|p| PaymentRes {
            id: p.payment_id,
            amount: p.amount,
            method_code: p.method_code,
            status: p.status,
        })
        .collect();

    Ok(HttpResponse::Ok()
        .append_header(("X-Total-Count", res.len().to_string()))
        .append_header(("X-Page", page.to_string()))
        .append_header(("X-Page-Size", size.to_string()))
        .json(res))
}


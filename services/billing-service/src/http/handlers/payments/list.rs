use actix_web::{web, HttpResponse, get};
use actix_web_validator::Query;
use uuid::Uuid;
use app_web::prelude::AuthUser;
use crate::infra::db::repositories::payment_repo;
use crate::http::dto::payment_dto::{PaymentQuery, PaymentRes};

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

    let (items, total) = payment_repo::list_paged(&db, q.invoice_id, page, size)
        .await
        .map_err(|e| {
            tracing::error!(?e, "list payments");
            crate::error::AppError::Internal("DB".into())
        })?;

    let res: Vec<PaymentRes> = items
        .into_iter()
        .map(|p| PaymentRes {
            id: p.id,
            pay_no: p.pay_no,
            amount: p.amount,
            method: p.method,
        })
        .collect();

    Ok(HttpResponse::Ok()
        .append_header(("X-Total-Count", total.to_string()))
        .append_header(("X-Page", page.to_string()))
        .append_header(("X-Page-Size", size.to_string()))
        .json(res))
}


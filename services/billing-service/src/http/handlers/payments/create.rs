use actix_web::{web, HttpResponse, post};
use actix_web_validator::Json;
use app_web::prelude::AuthUser;
use crate::domain::services::billing_svc;
use crate::http::dto::payment_dto::{CreatePaymentReq, PaymentRes};
use crate::infra::db::repositories::payment_repo;

#[post("/api/v1/payments:create")]
#[utoipa::path(
    post,
    path = "/api/v1/payments:create",
    request_body = CreatePaymentReq,
    security(("bearer_auth" = []))
)]
pub async fn create_payment(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: Json<CreatePaymentReq>,
    _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let id = billing_svc::create_payment(
        &db,
        payload.invoice_id,
        payload.method.clone(),
        payload.amount.clone(),
        payload.currency.as_deref().unwrap_or("VND").to_string(),
    )
    .await
    .map_err(|e| {
        tracing::error!(?e, "create payment");
        crate::error::AppError::Internal("DB".into())
    })?;

    let payment = payment_repo::find_by_id(&db, id)
        .await
        .map_err(|e| {
            tracing::error!(?e, "find payment");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;

    let res = PaymentRes {
        id: payment.id,
        pay_no: payment.pay_no,
        amount: payment.amount,
        method: payment.method,
    };

    Ok(HttpResponse::Created().json(res))
}


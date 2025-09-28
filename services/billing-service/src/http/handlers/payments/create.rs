use actix_web::{web, HttpResponse, post};
use actix_web_validator::Json;
use app_web::prelude::AuthUser;
use crate::http::dto::payment_dto::{CreatePaymentReq, PaymentRes};
use crate::http::handlers::common::create_billing_service;

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
    let billing_service = create_billing_service(&db);

    let payment_id = billing_service.create_payment(
        payload.invoice_id,
        payload.method_code.clone(),
        payload.amount,
        payload.ref_no.clone(),
    )
    .await
    .map_err(|e| {
        tracing::error!(?e, "create payment");
        crate::error::AppError::Internal("DB".into())
    })?;

    let payment = billing_service.get_payment(payment_id).await
        .map_err(|e| {
            tracing::error!(?e, "get payment");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;

    let res = PaymentRes {
        id: payment.payment_id,
        amount: payment.amount,
        method_code: payment.method_code,
        status: payment.status,
    };

    Ok(HttpResponse::Created().json(res))
}


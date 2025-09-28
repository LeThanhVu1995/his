use actix_web::{web, HttpResponse, post};
use actix_web_validator::Json;
use app_web::prelude::AuthUser;
use crate::http::dto::refund_dto::{CreateRefundReq, RefundRes};
use crate::http::handlers::common::create_billing_service;

#[post("/api/v1/refunds:create")]
#[utoipa::path(
    post,
    path = "/api/v1/refunds:create",
    request_body = CreateRefundReq,
    security(("bearer_auth" = []))
)]
pub async fn create_refund(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: Json<CreateRefundReq>,
    _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let billing_service = create_billing_service(&db);

    let refund_id = billing_service.create_refund(
        payload.payment_id,
        payload.amount,
        payload.reason.clone(),
        payload.ref_no.clone(),
    )
    .await
    .map_err(|e| {
        tracing::error!(?e, "create refund");
        crate::error::AppError::Internal("DB".into())
    })?;

    let refund = billing_service.get_refund(refund_id).await
        .map_err(|e| {
            tracing::error!(?e, "get refund");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;

    let res = RefundRes {
        id: refund.refund_id,
        payment_id: refund.payment_id,
        amount: refund.amount,
        reason: refund.reason,
        status: refund.status,
    };

    Ok(HttpResponse::Created().json(res))
}

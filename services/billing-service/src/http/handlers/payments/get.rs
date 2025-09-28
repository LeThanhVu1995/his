use actix_web::{web, HttpResponse, get};
use uuid::Uuid;
use app_web::prelude::AuthUser;
use crate::http::dto::payment_dto::PaymentRes;
use crate::http::handlers::common::create_billing_service;

#[get("/api/v1/payments/{id}")]
#[utoipa::path(
    get,
    path = "/api/v1/payments/{id}",
    security(("bearer_auth" = []))
)]
pub async fn get_payment(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let payment_id = path.into_inner();
    let billing_service = create_billing_service(&db);

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

    Ok(HttpResponse::Ok().json(res))
}

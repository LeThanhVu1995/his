use actix_web::{web, HttpResponse, post};
use actix_web_validator::Json;
use app_web::prelude::AuthUser;
use crate::http::dto::charge_dto::{CreateChargeReq, ChargeRes};
use crate::http::handlers::common::create_billing_service;

#[post("/api/v1/charges:create")]
#[utoipa::path(
    post,
    path = "/api/v1/charges:create",
    request_body = CreateChargeReq,
    security(("bearer_auth" = []))
)]
pub async fn create_charge(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: Json<CreateChargeReq>,
    _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let billing_service = create_billing_service(&db);

    let charge_id = billing_service.create_charge(
        payload.encounter_id,
        payload.patient_id,
        payload.code.clone(),
        payload.description.clone(),
        payload.qty,
        payload.unit_price,
    )
    .await
    .map_err(|e| {
        tracing::error!(?e, "create charge");
        crate::error::AppError::Internal("DB".into())
    })?;

    let charge = billing_service.get_charge(charge_id)
        .await
        .map_err(|e| {
            tracing::error!(?e, "find charge");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;

    let res = ChargeRes {
        id: charge.charge_id,
        code: charge.service_code,
        name: charge.description.unwrap_or_default(),
        qty: charge.qty,
        unit_price: charge.unit_price,
        amount: charge.amount,
        status: charge.status,
    };

    Ok(HttpResponse::Created().json(res))
}

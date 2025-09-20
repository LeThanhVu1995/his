use actix_web::{web, HttpResponse, post};
use actix_web_validator::Json;
use app_web::prelude::AuthUser;
use crate::domain::services::billing_svc;
use crate::http::dto::charge_dto::{CreateChargeReq, ChargeRes};
use crate::infra::db::repositories::charge_repo;

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
    let id = billing_svc::create_charge(
        &db,
        payload.patient_id,
        payload.encounter_id,
        payload.order_id,
        payload.code.clone(),
        payload.name.clone(),
        payload.qty.clone(),
        payload.unit_price.clone(),
        payload.currency.clone(),
    )
    .await
    .map_err(|e| {
        tracing::error!(?e, "create charge");
        crate::error::AppError::Internal("DB".into())
    })?;

    let charge = charge_repo::find_by_id(&db, id)
        .await
        .map_err(|e| {
            tracing::error!(?e, "find charge");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;

    let res = ChargeRes {
        id: charge.id,
        code: charge.code,
        name: charge.name,
        qty: charge.qty,
        unit_price: charge.unit_price,
        amount: charge.amount,
        status: charge.status,
    };

    Ok(HttpResponse::Created().json(res))
}

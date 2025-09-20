use actix_web::{web, HttpResponse, put};
use actix_web_validator::Json;
use uuid::Uuid;
use app_web::prelude::AuthUser;
use crate::infra::db::repositories::charge_repo;
use crate::http::dto::charge_dto::{UpdateChargeReq, ChargeRes};

#[put("/api/v1/charges/{id}")]
#[utoipa::path(
    put,
    path = "/api/v1/charges/{id}",
    request_body = UpdateChargeReq,
    security(("bearer_auth" = []))
)]
pub async fn update_charge(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: Json<UpdateChargeReq>,
    _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let id = path.into_inner();

    let charge = charge_repo::update(
        &db,
        id,
        payload.name.as_deref(),
        payload.qty,
        payload.unit_price,
        payload.status.as_deref(),
    )
    .await
    .map_err(|e| {
        tracing::error!(?e, "update charge");
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

    Ok(HttpResponse::Ok().json(res))
}


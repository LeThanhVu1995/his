use actix_web::{web, HttpResponse, put};
use actix_web_validator::Json;
use uuid::Uuid;
use app_web::prelude::AuthUser;
use crate::http::dto::charge_dto::{UpdateChargeReq, ChargeRes};
use crate::http::handlers::common::create_billing_service;

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

    let billing_service = create_billing_service(&db);

    // For now, just update status if provided
    if let Some(status) = &payload.status {
        billing_service.update_charge_status(id, status).await
            .map_err(|e| {
                tracing::error!(?e, "update charge status");
                crate::error::AppError::Internal("DB".into())
            })?;
    }

    let charge = billing_service.get_charge(id).await
        .map_err(|e| {
            tracing::error!(?e, "get charge");
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

    Ok(HttpResponse::Ok().json(res))
}


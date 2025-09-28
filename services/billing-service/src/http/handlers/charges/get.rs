use actix_web::{web, HttpResponse, get};
use uuid::Uuid;
use app_web::prelude::AuthUser;
use crate::http::dto::charge_dto::ChargeRes;
use crate::http::handlers::common::create_billing_service;

#[get("/api/v1/charges/{id}")]
#[utoipa::path(
    get,
    path = "/api/v1/charges/{id}",
    security(("bearer_auth" = []))
)]
pub async fn get_charge(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let charge_id = path.into_inner();
    let billing_service = create_billing_service(&db);

    let charge = billing_service.get_charge(charge_id).await
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

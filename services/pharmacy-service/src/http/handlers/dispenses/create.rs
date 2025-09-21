use actix_web::{web, HttpResponse};
use actix_web_validator::Json;

use app_web::prelude::AuthUser;
use crate::domain::service::PharmacyService;
use crate::http::dto::dispense_dto::{CreateDispenseReq, DispenseRes};


#[utoipa::path(
    post,
    path = "/api/v1/dispenses:create",
    request_body = CreateDispenseReq,
    responses(
        (status = 201, description = "Dispense created successfully", body = DispenseRes),
        (status = 400, description = "Bad request"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_dispense(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    payload: Json<CreateDispenseReq>,
    _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let svc = PharmacyService {
        meds: crate::domain::repositories::MedRepo { db: &db },
        presc: crate::domain::repositories::PrescriptionRepo { db: &db },
        items: crate::domain::repositories::PrescriptionItemRepo { db: &db },
        disp: crate::domain::repositories::DispenseRepo { db: &db },
    };

    let id = svc
        .create_dispense(payload.prescription_id, None)
        .await
        .map_err(|e| {
            tracing::error!(?e, "create dispense");
            crate::error::AppError::Internal("DB".into())
        })?;

    let repo = crate::domain::repositories::DispenseRepo { db: &db };
    let d = repo
        .find(id)
        .await
        .map_err(|e| {
            tracing::error!(?e, "find dispense");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;

    let res = DispenseRes {
        id: d.id,
        prescription_id: d.prescription_id,
        disp_no: d.disp_no,
        dispensed_by: d.dispensed_by,
        status: d.status,
    };

    Ok(HttpResponse::Created().json(res))
}

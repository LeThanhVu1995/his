use actix_web::{web, HttpResponse};
use uuid::Uuid;
use app_web::prelude::AuthUser;
use crate::domain::repositories::DispenseRepo;
use crate::http::dto::dispense_dto::DispenseRes;


#[utoipa::path(
    put,
    path = "/api/v1/dispenses/{id}:finish",
    params(("id" = Uuid, Path, description = "Dispense ID")),
    responses(
        (status = 200, description = "Dispense finished successfully", body = DispenseRes),
        (status = 404, description = "Dispense not found"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn finish_dispense(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
    let id = path.into_inner();
    let repo = DispenseRepo { db: &db };
    let rec = repo
        .finish(id)
        .await
        .map_err(|e| {
            tracing::error!(?e, "finish dispense");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;

    let res = DispenseRes {
        id: rec.id,
        prescription_id: rec.prescription_id,
        disp_no: rec.disp_no,
        dispensed_by: rec.dispensed_by,
        status: rec.status,
    };

    Ok(HttpResponse::Ok().json(res))
}

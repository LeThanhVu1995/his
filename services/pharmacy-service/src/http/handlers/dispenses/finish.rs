use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;

use app_web::prelude::AuthUser;
// Dispense import not needed for this handler
use crate::domain::repositories::DispenseRepo;

#[utoipa::path(
    put,
    path = "/api/v1/dispenses/{id}:finish",
    responses(
        (status = 200, description = "Dispense finished successfully", body = Dispense),
        (status = 404, description = "Dispense not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn finish_dispense(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    _auth_user: AuthUser,
) -> Result<HttpResponse> {
    let dispense_id = path.into_inner();
    let repo = DispenseRepo { db: &db };

    repo.get_by_id(dispense_id).await
        .map_err(|e| {
            tracing::error!(?e, "get dispense");
            crate::error::AppError::Internal("DB error".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;

    let finished_dispense = repo.finish(dispense_id).await
        .map_err(|e| {
            tracing::error!(?e, "finish dispense");
            crate::error::AppError::Internal("Failed to finish dispense".into())
        })?;

    Ok(HttpResponse::Ok().json(finished_dispense))
}
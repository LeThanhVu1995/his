use actix_web::{web, HttpResponse, Result};
use actix_web::web::Json;
use uuid::Uuid;
use validator::Validate;

use app_web::prelude::AuthUser;
use crate::domain::entities::prescription::UpdatePrescriptionRequest;
use crate::domain::repositories::PrescriptionRepo;

#[utoipa::path(
    put,
    path = "/api/v1/prescriptions/{id}",
    request_body = UpdatePrescriptionRequest,
    responses(
        (status = 200, description = "Prescription updated successfully", body = Prescription),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "Prescription not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_prescription(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    body: Json<UpdatePrescriptionRequest>,
    _auth_user: AuthUser,
) -> Result<HttpResponse> {
    if let Err(e) = body.validate() {
        return Err(crate::error::AppError::BadRequest(e.to_string()).into());
    }

    let prescription_id = path.into_inner();
    let repo = PrescriptionRepo { db: &db };

    repo.get_by_id(prescription_id).await
        .map_err(|e| {
            tracing::error!(?e, "get prescription");
            crate::error::AppError::Internal("DB error".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;

    let updated_prescription = repo.update(prescription_id, &body).await
        .map_err(|e| {
            tracing::error!(?e, "update prescription");
            crate::error::AppError::Internal("Failed to update prescription".into())
        })?;

    Ok(HttpResponse::Ok().json(updated_prescription))
}
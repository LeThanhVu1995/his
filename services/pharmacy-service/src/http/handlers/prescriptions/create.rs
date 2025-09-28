use actix_web::{web, HttpResponse, Result};
use actix_web::web::Json;
use validator::Validate;

use app_web::prelude::AuthUser;
use crate::domain::entities::prescription::CreatePrescriptionRequest;
use crate::domain::repositories::PrescriptionRepo;

#[utoipa::path(
    post,
    path = "/api/v1/prescriptions:create",
    request_body = CreatePrescriptionRequest,
    responses(
        (status = 201, description = "Prescription created successfully", body = Prescription),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_prescription(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: Json<CreatePrescriptionRequest>,
    auth_user: AuthUser,
) -> Result<HttpResponse> {
    if let Err(e) = body.validate() {
        return Err(crate::error::AppError::BadRequest(e.to_string()).into());
    }

    let repo = PrescriptionRepo { db: &db };
    let new_prescription = repo.create(&body).await
        .map_err(|e| {
            tracing::error!(?e, "create prescription");
            crate::error::AppError::Internal("Failed to create prescription".into())
        })?;

    // Create prescription items
    for item in &body.items {
        repo.create_item(new_prescription.prescription_id, item).await
            .map_err(|e| {
                tracing::error!(?e, "create prescription item");
                crate::error::AppError::Internal("Failed to create prescription item".into())
            })?;
    }

    Ok(HttpResponse::Created().json(new_prescription))
}
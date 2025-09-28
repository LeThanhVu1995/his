use actix_web::{web, HttpResponse, Result};
use actix_web::web::Json;
use validator::Validate;

use app_web::prelude::AuthUser;
use crate::domain::entities::dispense::CreateDispenseRequest;
use crate::domain::repositories::DispenseRepo;

#[utoipa::path(
    post,
    path = "/api/v1/dispenses:create",
    request_body = CreateDispenseRequest,
    responses(
        (status = 201, description = "Dispense created successfully", body = Dispense),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_dispense(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: Json<CreateDispenseRequest>,
    auth_user: AuthUser,
) -> Result<HttpResponse> {
    if let Err(e) = body.validate() {
        return Err(crate::error::AppError::BadRequest(e.to_string()).into());
    }

    let repo = DispenseRepo { db: &db };
    let new_dispense = repo.create(&body).await
        .map_err(|e| {
            tracing::error!(?e, "create dispense");
            crate::error::AppError::Internal("Failed to create dispense".into())
        })?;

    // Create dispense items
    for item in &body.items {
        repo.create_item(new_dispense.dispense_id, item).await
            .map_err(|e| {
                tracing::error!(?e, "create dispense item");
                crate::error::AppError::Internal("Failed to create dispense item".into())
            })?;
    }

    Ok(HttpResponse::Created().json(new_dispense))
}
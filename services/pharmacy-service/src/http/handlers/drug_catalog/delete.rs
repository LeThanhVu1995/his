use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;

use app_web::prelude::AuthUser;
use crate::domain::repositories::DrugCatalogRepo;

#[utoipa::path(
    delete,
    path = "/api/v1/pharmacy/drug-catalog/{id}",
    responses(
        (status = 204, description = "Drug deleted successfully"),
        (status = 404, description = "Drug not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_drug_catalog(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    auth_user: AuthUser,
) -> Result<HttpResponse> {
    let drug_id = path.into_inner();
    let repo = DrugCatalogRepo { db: &db };

    repo.get_by_id(drug_id).await
        .map_err(|e| {
            tracing::error!(?e, "get drug catalog");
            crate::error::AppError::Internal("DB error".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;

    let user_id = auth_user.user_id.parse::<Uuid>()
        .map_err(|_| crate::error::AppError::BadRequest("Invalid user ID".into()))?;
    repo.delete(drug_id, user_id).await
        .map_err(|e| {
            tracing::error!(?e, "delete drug catalog");
            crate::error::AppError::Internal("Failed to delete drug".into())
        })?;

    Ok(HttpResponse::NoContent().finish())
}

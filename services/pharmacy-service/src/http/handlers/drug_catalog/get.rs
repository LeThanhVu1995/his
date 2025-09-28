use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;

use app_web::prelude::AuthUser;
// DrugCatalog import not needed for this handler
use crate::domain::repositories::DrugCatalogRepo;

#[utoipa::path(
    get,
    path = "/api/v1/pharmacy/drug-catalog/{id}",
    responses(
        (status = 200, description = "Drug found", body = DrugCatalog),
        (status = 404, description = "Drug not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_drug_catalog(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    _auth_user: AuthUser,
) -> Result<HttpResponse> {
    let drug_id = path.into_inner();
    let repo = DrugCatalogRepo { db: &db };
    let drug = repo.get_by_id(drug_id).await
        .map_err(|e| {
            tracing::error!(?e, "get drug catalog");
            crate::error::AppError::Internal("DB error".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;

    Ok(HttpResponse::Ok().json(drug))
}

use actix_web::{web, HttpResponse, Result};

use app_web::prelude::AuthUser;
// DrugCatalogStats import not needed for this handler
use crate::domain::repositories::DrugCatalogRepo;

#[utoipa::path(
    get,
    path = "/api/v1/pharmacy/drug-catalog/stats",
    responses(
        (status = 200, description = "Drug catalog statistics", body = DrugCatalogStats),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_drug_catalog_stats(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    _auth_user: AuthUser,
) -> Result<HttpResponse> {
    let repo = DrugCatalogRepo { db: &db };
    let stats = repo.get_stats().await
        .map_err(|e| {
            tracing::error!(?e, "get drug catalog stats");
            crate::error::AppError::Internal("DB error".into())
        })?;

    Ok(HttpResponse::Ok().json(stats))
}

use actix_web::{web, HttpResponse, Result};
use actix_web::web::Json;
use validator::Validate;

use app_web::prelude::AuthUser;
use crate::domain::entities::drug_catalog::CreateDrugCatalogRequest;
use crate::domain::repositories::DrugCatalogRepo;

#[utoipa::path(
    post,
    path = "/api/v1/pharmacy/drug-catalog",
    request_body = CreateDrugCatalogRequest,
    responses(
        (status = 201, description = "Drug created successfully", body = DrugCatalog),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_drug_catalog(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: Json<CreateDrugCatalogRequest>,
    _auth_user: AuthUser,
) -> Result<HttpResponse> {
    if let Err(e) = body.validate() {
        return Err(crate::error::AppError::BadRequest(e.to_string()).into());
    }

    let repo = DrugCatalogRepo { db: &db };
    let new_drug = repo.create(&body).await
        .map_err(|e| {
            tracing::error!(?e, "create drug catalog");
            crate::error::AppError::Internal("Failed to create drug".into())
        })?;

    Ok(HttpResponse::Created().json(new_drug))
}

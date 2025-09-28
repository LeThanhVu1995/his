use actix_web::{web, HttpResponse, Result};
use actix_web::web::Json;
use uuid::Uuid;
use validator::Validate;

use app_web::prelude::AuthUser;
use crate::domain::entities::drug_catalog::UpdateDrugCatalogRequest;
use crate::domain::repositories::DrugCatalogRepo;

#[utoipa::path(
    put,
    path = "/api/v1/pharmacy/drug-catalog/{id}",
    request_body = UpdateDrugCatalogRequest,
    responses(
        (status = 200, description = "Drug updated successfully", body = DrugCatalog),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "Drug not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_drug_catalog(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    body: Json<UpdateDrugCatalogRequest>,
    _auth_user: AuthUser,
) -> Result<HttpResponse> {
    if let Err(e) = body.validate() {
        return Err(crate::error::AppError::BadRequest(e.to_string()).into());
    }

    let drug_id = path.into_inner();
    let repo = DrugCatalogRepo { db: &db };

    repo.get_by_id(drug_id).await
        .map_err(|e| {
            tracing::error!(?e, "get drug catalog");
            crate::error::AppError::Internal("DB error".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;

    let updated_drug = repo.update(drug_id, &body).await
        .map_err(|e| {
            tracing::error!(?e, "update drug catalog");
            crate::error::AppError::Internal("Failed to update drug".into())
        })?;

    Ok(HttpResponse::Ok().json(updated_drug))
}

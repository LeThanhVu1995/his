use actix_web::{web, HttpResponse, Result, HttpRequest};

use app_web::prelude::AuthUser;
use crate::domain::repositories::DrugCatalogRepo;

#[utoipa::path(
    get,
    path = "/api/v1/pharmacy/drug-catalog",
    responses(
        (status = 200, description = "List of drugs"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_drug_catalog(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    req: HttpRequest,
    _auth_user: AuthUser,
) -> Result<HttpResponse> {
    let query = web::Query::<std::collections::HashMap<String, String>>::from_query(req.query_string())
        .unwrap_or(web::Query(std::collections::HashMap::new()));
    
    let page = query.get("page")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(1);
    let page_size = query.get("page_size")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(10);
    let offset = (page - 1) * page_size;

    let code = query.get("code").cloned();
    let name = query.get("name").cloned();
    let atc_code = query.get("atc_code").cloned();
    let form_code = query.get("form_code").cloned();

    let repo = DrugCatalogRepo { db: &db };
    let drugs = repo.list_paged(
        code,
        name,
        atc_code,
        form_code,
        page_size,
        offset
    ).await
        .map_err(|e| {
            tracing::error!(?e, "list drug catalog");
            crate::error::AppError::Internal("DB error".into())
        })?;

    Ok(HttpResponse::Ok().json(drugs))
}

use actix_web::{web, HttpResponse, Result, HttpRequest};
use uuid::Uuid;

use app_web::prelude::AuthUser;
use crate::domain::repositories::DispenseRepo;

#[utoipa::path(
    get,
    path = "/api/v1/dispenses",
    responses(
        (status = 200, description = "List of dispenses"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_dispenses(
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

    let prescription_id = query.get("prescription_id")
        .and_then(|s| s.parse::<Uuid>().ok());
    let dispensed_by = query.get("dispensed_by")
        .and_then(|s| s.parse::<Uuid>().ok());
    let status = query.get("status").cloned();

    let repo = DispenseRepo { db: &db };
    let dispenses = repo.list_paged(
        prescription_id,
        dispensed_by,
        status,
        page_size,
        offset
    ).await
        .map_err(|e| {
            tracing::error!(?e, "list dispenses");
            crate::error::AppError::Internal("DB error".into())
        })?;

    Ok(HttpResponse::Ok().json(dispenses))
}
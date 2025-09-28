use actix_web::{web, HttpResponse, Result, HttpRequest};
use uuid::Uuid;

use app_web::prelude::AuthUser;
use crate::domain::repositories::PrescriptionRepo;

#[utoipa::path(
    get,
    path = "/api/v1/prescriptions",
    responses(
        (status = 200, description = "List of prescriptions"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn list_prescriptions(
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

    let patient_id = query.get("patient_id")
        .and_then(|s| s.parse::<Uuid>().ok());
    let encounter_id = query.get("encounter_id")
        .and_then(|s| s.parse::<Uuid>().ok());
    let prescriber_id = query.get("prescriber_id")
        .and_then(|s| s.parse::<Uuid>().ok());
    let status = query.get("status").cloned();

    let repo = PrescriptionRepo { db: &db };
    let prescriptions = repo.list_paged(
        patient_id,
        encounter_id,
        prescriber_id,
        status,
        page_size,
        offset
    ).await
        .map_err(|e| {
            tracing::error!(?e, "list prescriptions");
            crate::error::AppError::Internal("DB error".into())
        })?;

    Ok(HttpResponse::Ok().json(prescriptions))
}
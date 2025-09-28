use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;
use crate::domain::entities::study::UpdateStudyRequest;

#[utoipa::path(
    put,
    path = "/api/v1/ris-pacs/studies/{id}/start",
    params(
        ("id" = Uuid, Path, description = "Study ID")
    ),
    request_body = UpdateStudyRequest,
    responses(
        (status = 200, description = "Study started successfully"),
        (status = 404, description = "Study not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn start_study(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
    payload: web::Json<UpdateStudyRequest>,
) -> Result<HttpResponse> {
    // TODO: Implement study start
    // 1. Validate study exists and is in SCHEDULED status
    // 2. Update study status to IN_PROGRESS
    // 3. Set started_at timestamp
    // 4. Update performing physician if provided
    // 5. Return success
    
    Ok(HttpResponse::Ok().json("Study started"))
}
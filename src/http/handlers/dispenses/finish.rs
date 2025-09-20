use actix_web::{put, web, HttpResponse};
use uuid::Uuid;
use crate::domain::repo::DispenseRepo;
use crate::http::dto::dispense_dto::DispenseRes;

#[put("/api/v1/dispenses/{id}:finish")]
pub async fn finish_dispense(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let repo = DispenseRepo { db: &db };
    let rec = repo
        .finish(path.into_inner())
        .await
        .map_err(|e| {
            tracing::error!(?e, "finish disp");
            crate::error::AppError::Internal("DB".into())
        })?
        .ok_or(crate::error::AppError::NotFound)?;
    let res = DispenseRes {
        id: rec.id,
        disp_no: rec.disp_no,
        status: rec.status,
    };
    Ok(HttpResponse::Ok().json(res))
}

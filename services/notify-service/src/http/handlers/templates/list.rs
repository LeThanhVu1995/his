use actix_web::{web, HttpResponse};
use crate::infrastructure::repositories::template_repo::TemplateRepo;

pub async fn list(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
) -> actix_web::Result<HttpResponse> {
    let (items, total) = TemplateRepo { db: &db }.list(None, 1, 200).await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Ok(HttpResponse::Ok()
        .append_header(("X-Total-Count", total.to_string()))
        .json(items))
}

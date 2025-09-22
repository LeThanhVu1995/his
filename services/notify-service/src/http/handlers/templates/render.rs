use actix_web::{web, HttpResponse};
use crate::domain::services::render_svc::RenderSvc;

pub async fn render(
    _db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<crate::dto::template_dto::RenderReq>,
) -> actix_web::Result<HttpResponse> {
    let html = RenderSvc::new().render(&body.body, &body.variables)
        .map_err(|_| crate::error::AppError::BadRequest("render".into()))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "content": html
    })))
}

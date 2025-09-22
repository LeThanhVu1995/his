use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::infrastructure::repositories::template_repo::TemplateRepo;
use crate::domain::entities::template::Template;

pub async fn create(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<crate::dto::template_dto::CreateTemplateReq>,
) -> actix_web::Result<HttpResponse> {
    let t = Template {
        id: Uuid::new_v4(),
        code: body.code.clone(),
        name: body.name.clone(),
        channel: body.channel.clone(),
        subject: body.subject.clone(),
        body: body.body.clone(),
        is_active: true,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    TemplateRepo { db: &db }.create(&t).await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Ok(HttpResponse::Created().json(crate::dto::template_dto::TemplateRes {
        id: t.id,
        code: t.code,
        channel: t.channel,
    }))
}

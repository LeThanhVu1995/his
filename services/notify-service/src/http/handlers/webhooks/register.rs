use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::{infrastructure::repositories::webhook_repo::WebhookRepo, domain::entities::webhook::Webhook};

pub async fn register(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<crate::dto::webhook_dto::RegisterWebhookReq>,
) -> actix_web::Result<HttpResponse> {
    let w = Webhook {
        id: Uuid::new_v4(),
        name: body.name.clone(),
        url: body.url.clone(),
        secret: body.secret.clone(),
        is_active: true,
        created_at: chrono::Utc::now(),
    };

    WebhookRepo { db: &db }.register(&w).await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Ok(HttpResponse::Created().json(crate::dto::webhook_dto::RegisterWebhookRes { id: w.id }))
}

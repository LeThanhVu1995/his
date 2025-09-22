use actix_web::{web, HttpResponse};
use crate::infrastructure::repositories::webhook_repo::WebhookRepo;

pub async fn trigger(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<serde_json::Value>,
) -> actix_web::Result<HttpResponse> {
    let hooks = WebhookRepo { db: &db }.list().await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    for h in hooks {
        let _ = reqwest::Client::new()
            .post(&h.url)
            .json(&*body)
            .send()
            .await;
    }

    Ok(HttpResponse::Ok().finish())
}

use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::store::instances::InstanceStore;
use crate::engine::interpreter::Interpreter;
pub async fn start(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<String>,
    body: web::Json<serde_json::Value>,
) -> actix_web::Result<HttpResponse> {
    let id = InstanceStore { db: &db }
        .create(&path, &*body)
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?;

    Interpreter { db: &db }
        .tick(id)
        .await
        .map_err(|e| {
            tracing::error!(?e, "wf tick");
            crate::error::AppError::Internal("tick".into())
        })?;

    Ok(HttpResponse::Created().json(serde_json::json!({"id": id})))
}

pub async fn get(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    path: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    let ins = InstanceStore { db: &db }
        .get(path.into_inner())
        .await
        .map_err(|_| crate::error::AppError::Internal("DB".into()))?
        .ok_or(crate::error::AppError::NotFound)?;

    Ok(HttpResponse::Ok().json(ins))
}

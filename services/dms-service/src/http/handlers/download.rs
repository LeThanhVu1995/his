use actix_web::{post, web, HttpResponse};
use app_web::extractors::auth_user::AuthUser;

#[post("/api/v1/dms/objects:presign-download")]
pub async fn presign_download(
  _user: AuthUser,
  _db: web::Data<sqlx::Pool<sqlx::Postgres>>,
  body: web::Json<crate::http::dto::object_dto::GetUrlReq>
) -> actix_web::Result<HttpResponse> {
  let s3=crate::infra::storage::minio::S3::from_env().await.map_err(|e|{ tracing::error!(error=%e,"s3"); actix_web::error::ErrorInternalServerError("s3") })?;
  let url=s3.presign_get(&body.key, std::env::var("PRESIGN_EXPIRES_SECS").ok().and_then(|v|v.parse().ok()).unwrap_or(900)).await.map_err(|e|{ tracing::error!(error=%e,"presign"); actix_web::error::ErrorInternalServerError("presign") })?;
  Ok(HttpResponse::Ok().json(crate::http::dto::object_dto::GetUrlRes{ download_url:url }))
}

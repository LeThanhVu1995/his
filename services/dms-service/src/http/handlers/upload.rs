use actix_web::{post, web, HttpResponse};
use crate::domain::services::presign_svc::PresignSvc;
use app_web::extractors::auth_user::AuthUser;

#[post("/api/v1/dms/objects:presign-upload")]
pub async fn presign_upload(
  db: web::Data<sqlx::Pool<sqlx::Postgres>>,
  body: web::Json<crate::http::dto::object_dto::CreatePresignReq>,
  user: AuthUser,
) -> actix_web::Result<HttpResponse> {
  let s3=crate::infra::storage::minio::S3::from_env().await.map_err(|e|{ tracing::error!(error=%e,"s3"); actix_web::error::ErrorInternalServerError("s3") })?;
  let svc=PresignSvc{ s3, db:(**db).clone() };
  let user_uuid = user.user_id.parse::<uuid::Uuid>().map_err(|_| actix_web::error::ErrorBadRequest("Invalid user ID"))?;
  let (id,key,url)=svc.create_record_and_presign_put(&body.name, &body.content_type, body.category.as_deref(), (body.entity_type.as_deref(), body.entity_id), Some(user_uuid)).await.map_err(|e|{ tracing::error!(error=%e,"presign"); actix_web::error::ErrorInternalServerError("presign") })?;
  Ok(HttpResponse::Created().json(crate::http::dto::object_dto::CreatePresignRes{ id, key, upload_url:url }))
}

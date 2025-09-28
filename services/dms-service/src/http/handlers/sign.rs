use actix_web::{post, web, HttpResponse};
use crate::domain::services::sign_svc::SignSvc;
use app_web::extractors::auth_user::AuthUser;

#[post("/api/v1/dms/signatures:attach")]
pub async fn attach(
  db: web::Data<sqlx::Pool<sqlx::Postgres>>,
  body: web::Json<crate::http::dto::sign_dto::AttachSignReq>,
  _user: AuthUser,
) -> actix_web::Result<HttpResponse> {
  let id=SignSvc{ db:&**db }.attach_signature(body.object_id, body.signer_id, body.signer_name.clone(), &body.signature_alg, &body.signature_b64, body.note.clone()).await.map_err(|e|{ tracing::error!(error=%e,"sign"); actix_web::error::ErrorInternalServerError("sign") })?;
  Ok(HttpResponse::Created().json(serde_json::json!({"id":id})))
}

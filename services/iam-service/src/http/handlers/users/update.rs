// iam-service src/http/handlers/users/update.rs placeholder
use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::infra::db::pool::PgPool;
use crate::domain::services::user_svc;
use crate::http::dto::user_dto::UserUpdateReq;
use app_web::prelude::AuthUser;
use crate::rbac;

#[utoipa::path(
    put,
    path = "/api/iam/users/{id}",
    tag = "iam",
    request_body = UserUpdateReq,
    params(("id" = Uuid, Path, description = "User id")),
    responses((status = 200, description = "Updated user")),
    security(("bearerAuth" = ["iam.user.update"]))
)]
pub async fn update(db: web::Data<PgPool>, path: web::Path<Uuid>, payload: web::Json<UserUpdateReq>, user: AuthUser) -> Result<HttpResponse, app_error::AppError> {
    rbac::require(&user, &[rbac::ADMIN], &[rbac::IAM_USER_UPDATE])?;

    let id = path.into_inner();
    let u = user_svc::update_user(db.get_ref(), id, payload.full_name.clone(), payload.email.clone(), payload.locked).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "id": u.id,
        "username": u.username,
        "full_name": u.full_name,
        "email": u.email,
        "locked": u.locked
    })))
}

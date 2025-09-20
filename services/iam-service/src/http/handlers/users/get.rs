// iam-service src/http/handlers/users/get.rs placeholder
use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::domain::services::user_svc;
use crate::http::dto::user_dto::UserDto;
use crate::infra::db::pool::PgPool;
use app_web::prelude::AuthUser;
use crate::rbac;

#[utoipa::path(
    get,
    path = "/api/iam/users/{id}",
    tag = "iam",
    params(("id" = Uuid, Path, description = "User id")),
    responses((status = 200, description = "User")),
    security(("bearerAuth" = ["iam.user.read"]))
)]
pub async fn get(db: web::Data<PgPool>, path: web::Path<Uuid>, user: AuthUser) -> Result<HttpResponse, app_error::AppError> {
    rbac::require(&user, &[rbac::ADMIN], &[rbac::IAM_USER_READ])?;

    let id = path.into_inner();
    let u = user_svc::get_user(db.get_ref(), id).await?;
    let dto = UserDto { id: u.id, username: u.username, full_name: u.full_name, email: u.email, locked: u.locked };
    Ok(HttpResponse::Ok().json(dto))
}

// iam-service src/http/handlers/users/create.rs placeholder
use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::domain::services::user_svc;
use crate::http::dto::user_dto::{UserCreateReq, UserDto};
use crate::infra::db::pool::PgPool;
use app_web::prelude::AuthUser;
use crate::rbac;

#[utoipa::path(
    post,
    path = "/api/iam/users",
    tag = "iam",
    request_body = UserCreateReq,
    responses((status = 201, description = "Created user")),
    security(("bearerAuth" = ["iam.user.create"]))
)]
pub async fn create(db: web::Data<PgPool>, payload: web::Json<UserCreateReq>, user: AuthUser) -> Result<HttpResponse, app_error::AppError> {
    rbac::require(&user, &[rbac::ADMIN], &[rbac::IAM_USER_CREATE])?;

    let u = user_svc::create_user(db.get_ref(), &payload.username, payload.full_name.as_deref(), payload.email.as_deref()).await?;
    let dto = UserDto { id: u.id, username: u.username, full_name: u.full_name, email: u.email, locked: u.locked };
    Ok(HttpResponse::Created().json(dto))
}

use actix_web::{web, HttpResponse};
use crate::infra::db::pool::PgPool;
use crate::domain::services::role_svc;
use crate::http::dto::role_dto::AssignRoleReq;
use app_web::prelude::AuthUser;
use crate::rbac;

#[utoipa::path(
    post,
    path = "/api/iam/roles/assign",
    tag = "iam",
    request_body = AssignRoleReq,
    responses((status = 204, description = "Assigned")),
    security(("bearerAuth" = ["iam.role.assign"]))
)]
pub async fn assign(db: web::Data<PgPool>, payload: web::Json<AssignRoleReq>, user: AuthUser) -> Result<HttpResponse, app_error::AppError> {
    rbac::require(&user, &[rbac::ADMIN], &[rbac::IAM_ROLE_ASSIGN])?;
    role_svc::assign_role(db.get_ref(), payload.user_id, payload.role_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

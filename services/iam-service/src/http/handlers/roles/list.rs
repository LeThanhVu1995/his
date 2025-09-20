use actix_web::{web, HttpResponse};
use crate::infra::db::pool::PgPool;
use crate::domain::services::role_svc;
use app_web::prelude::AuthUser;
use crate::rbac;

#[utoipa::path(
    get,
    path = "/api/iam/roles",
    tag = "iam",
    responses((status = 200, description = "List roles")),
    security(("bearerAuth" = ["iam.role.read"]))
)]
pub async fn list(db: web::Data<PgPool>, user: AuthUser) -> Result<HttpResponse, app_error::AppError> {
    rbac::require(&user, &[rbac::ADMIN], &[rbac::IAM_ROLE_READ])?;
    let items = role_svc::list_roles(db.get_ref()).await?;
    Ok(HttpResponse::Ok().json(items))
}

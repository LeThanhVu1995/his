use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::infra::db::pool::PgPool;
use app_web::prelude::AuthUser;
use crate::rbac;

#[utoipa::path(delete, path = "/api/iam/policies/{id}", tag = "iam", params(("id"=Uuid, Path,)), responses((status=204)), security(("bearerAuth" = ["iam.policy.delete"])))]
pub async fn delete(db: web::Data<PgPool>, path: web::Path<Uuid>, user: AuthUser) -> Result<HttpResponse, app_error::AppError> {
    rbac::require(&user, &[rbac::ADMIN], &["iam.policy.delete"]) ?;
    crate::infra::db::repositories::policy_repo::delete(db.get_ref(), path.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}



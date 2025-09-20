use actix_web::{web, HttpResponse};
use crate::infra::db::pool::PgPool;
use app_web::prelude::AuthUser;
use crate::rbac;

#[utoipa::path(get, path = "/api/iam/policies", tag = "iam", responses((status=200)), security(("bearerAuth"=["iam.policy.read"])))]
pub async fn list(db: web::Data<PgPool>, user: AuthUser) -> Result<HttpResponse, app_error::AppError> {
    rbac::require(&user, &[rbac::ADMIN], &["iam.policy.read"]) ?;
    let items = crate::infra::db::repositories::policy_repo::list(db.get_ref()).await?;
    Ok(HttpResponse::Ok().json(items))
}



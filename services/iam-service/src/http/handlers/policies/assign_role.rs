use actix_web::{web, HttpResponse};
use crate::infra::db::pool::PgPool;
use app_web::prelude::AuthUser;
use crate::http::dto::policy_dto::AssignPolicyToRoleReq;
use crate::rbac;

#[utoipa::path(post, path = "/api/iam/policies/assign/role", tag = "iam", request_body = AssignPolicyToRoleReq, responses((status=204)), security(("bearerAuth"=["iam.policy.assign"])))]
pub async fn assign_role(db: web::Data<PgPool>, payload: web::Json<AssignPolicyToRoleReq>, user: AuthUser) -> Result<HttpResponse, app_error::AppError> {
    rbac::require(&user, &[rbac::ADMIN], &["iam.policy.assign"]) ?;
    crate::infra::db::repositories::policy_repo::assign_to_role(db.get_ref(), payload.role_id, payload.policy_id).await?;
    Ok(HttpResponse::NoContent().finish())
}



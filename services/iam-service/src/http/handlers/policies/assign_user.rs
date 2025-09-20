use actix_web::{web, HttpResponse};
use crate::infra::db::pool::PgPool;
use app_web::prelude::AuthUser;
use crate::http::dto::policy_dto::AssignPolicyToUserReq;
use crate::rbac;

#[utoipa::path(post, path = "/api/iam/policies/assign/user", tag = "iam", request_body = AssignPolicyToUserReq, responses((status=204)), security(("bearerAuth"=["iam.policy.assign"])))]
pub async fn assign_user(db: web::Data<PgPool>, payload: web::Json<AssignPolicyToUserReq>, user: AuthUser) -> Result<HttpResponse, app_error::AppError> {
    rbac::require(&user, &[rbac::ADMIN], &["iam.policy.assign"]) ?;
    crate::infra::db::repositories::policy_repo::assign_to_user(db.get_ref(), payload.user_id, payload.policy_id).await?;
    Ok(HttpResponse::NoContent().finish())
}



use actix_web::{web, HttpResponse};
use crate::infra::db::pool::PgPool;
use app_web::prelude::AuthUser;
use crate::http::dto::policy_dto::PolicyCreateReq;
use crate::rbac;

#[utoipa::path(post, path = "/api/iam/policies", tag = "iam", request_body = PolicyCreateReq, responses((status=201)), security(("bearerAuth"=["iam.policy.create"])))]
pub async fn create(db: web::Data<PgPool>, payload: web::Json<PolicyCreateReq>, user: AuthUser) -> Result<HttpResponse, app_error::AppError> {
    rbac::require(&user, &[rbac::ADMIN], &["iam.policy.create"]) ?;
    let rec = crate::infra::db::repositories::policy_repo::create(db.get_ref(), payload.into_inner()).await?;
    Ok(HttpResponse::Created().json(rec))
}



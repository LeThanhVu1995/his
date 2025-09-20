// iam-service src/http/handlers/users/lock.rs placeholder
use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::infra::db::pool::PgPool;
use crate::domain::services::user_svc;
use app_web::prelude::AuthUser;
use crate::rbac;

#[utoipa::path(
    post,
    path = "/api/iam/users/{id}/lock",
    tag = "iam",
    params(("id" = Uuid, Path, description = "User id")),
    responses((status = 200, description = "Locked status")),
    security(("bearerAuth" = ["iam.user.lock"]))
)]
pub async fn lock(db: web::Data<PgPool>, path: web::Path<Uuid>, user: AuthUser) -> Result<HttpResponse, app_error::AppError> {
    rbac::require(&user, &[rbac::ADMIN], &[rbac::IAM_USER_LOCK])?;

    let id = path.into_inner();
    let u = user_svc::set_lock(db.get_ref(), id, true).await?;
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "id": u.id,
        "locked": u.locked
    })))
}

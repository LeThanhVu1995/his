// iam-service src/http/handlers/users/list.rs placeholder
use actix_web::{web, HttpResponse};
use crate::infra::db::pool::PgPool;
use crate::domain::services::user_svc;
use crate::http::dto::user_dto::UserDto;
use app_core::prelude::*;
use serde::Deserialize;
use app_web::prelude::AuthUser;
use crate::rbac;

#[derive(Debug, Clone, Deserialize)]
pub struct PageQuery { pub page: Option<u32>, pub page_size: Option<u32> }

#[utoipa::path(
    get,
    path = "/api/iam/users",
    tag = "iam",
    params(
        ("page" = Option<u32>, Query, description = "Page number, default 1"),
        ("page_size" = Option<u32>, Query, description = "Page size, default 20"),
    ),
    responses((status = 200, description = "Paged users")),
    security(("bearerAuth" = ["iam.user.read"]))
)]
pub async fn list(db: web::Data<PgPool>, q: web::Query<PageQuery>, user: AuthUser) -> Result<HttpResponse, app_error::AppError> {
    rbac::require(&user, &[rbac::ADMIN], &[rbac::IAM_USER_READ])?;

    let page = PageParams { page: q.page.unwrap_or(1), page_size: q.page_size.unwrap_or(20) };
    let clamp = PaginationClamp::default();
    let (page_no, page_size, offset, limit) = page.clamp(clamp);

    let (items, total) = user_svc::list_users(db.get_ref(), offset as i64, limit as i64).await?;
    let data: Vec<UserDto> = items.into_iter().map(|u| UserDto { id: u.id, username: u.username, full_name: u.full_name, email: u.email, locked: u.locked }).collect();

    let resp = to_paged(data, total as u64, page_no, page_size);
    Ok(HttpResponse::Ok().json(resp))
}

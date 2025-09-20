use actix_web::{HttpResponse, Responder, HttpRequest, web};
use app_web::prelude::AuthUser;
use crate::infra::db::pool::PgPool;
use crate::infra::db::repositories::audit_repo;

#[utoipa::path(
    get,
    path = "/api/iam/me",
    tag = "iam",
    responses((status = 200, description = "Current user info")),
    security(("bearerAuth" = []))
)]
pub async fn me(req: HttpRequest, db: web::Data<PgPool>, user: AuthUser) -> impl Responder {
    let ip = req
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.split(',').next().unwrap_or(s).trim().to_string())
        .or_else(|| req.peer_addr().map(|p| p.ip().to_string()));
    let ua = req
        .headers()
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let db2 = db.clone();
    let uid = user.user_id.clone();
    let uname = user.preferred_username.clone();
    actix_rt::spawn(async move {
        let _ = audit_repo::insert_login(db2.get_ref(), &uid, uname.as_deref(), ip.as_deref(), ua.as_deref(), true).await;
    });

    HttpResponse::Ok().json(serde_json::json!({
        "user_id": user.user_id,
        "username": user.preferred_username,
        "roles": user.roles,
        "scopes": user.scopes,
    }))
}

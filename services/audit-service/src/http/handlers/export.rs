use actix_web::{web, HttpResponse};
use sqlx::Row;

pub async fn export_ndjson(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    _q: web::Query<serde_json::Value>,
) -> actix_web::Result<HttpResponse> {
    let rows = sqlx::query(
        r#"SELECT to_jsonb(t) AS j FROM (
           SELECT id,occurred_at,actor_id,actor_name,actor_role,ip::text as ip,user_agent,action,entity_type,entity_id,tenant_id,request_id,source,data,hash,created_at
           FROM audit_events ORDER BY occurred_at DESC LIMIT 10000) t"#
    )
    .fetch_all(&**db)
    .await
    .map_err(|_| actix_web::error::ErrorInternalServerError("db"))?;

    let mut body = String::new();
    for r in rows {
        // try column by index 0
        let v: Result<serde_json::Value, _> = r.try_get(0);
        if let Ok(j) = v { body.push_str(&j.to_string()); body.push('\n'); }
    }
    Ok(HttpResponse::Ok()
        .append_header(("Content-Type", "application/x-ndjson"))
        .append_header(("Content-Disposition", "attachment; filename=audit.ndjson"))
        .body(body))
}



use actix_web::{web, HttpResponse};
use sqlx::Row;

pub async fn export_ndjson(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    _q: web::Query<serde_json::Value>,
) -> actix_web::Result<HttpResponse> {
    let rows = sqlx::query(
        r#"SELECT to_jsonb(t) AS j FROM (
           SELECT audit_id, event_time, user_id, entity_name, entity_id, action, before_json, after_json, ip_address
           FROM audit_log ORDER BY event_time DESC LIMIT 10000) t"#
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



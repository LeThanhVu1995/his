use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::Row;

#[derive(Deserialize)]
pub struct AdhocReq {
    table: String,
    columns: Vec<String>,
    filters: Option<serde_json::Value>,
    limit: Option<i64>,
}

pub async fn adhoc(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<AdhocReq>,
) -> actix_web::Result<HttpResponse> {
    let allowed = vec![
        ("rpt.mv_revenue_daily", vec!["day", "revenue", "invoices"]),
        (
            "rpt.mv_visits_by_dept",
            vec!["department_id", "department_name", "day", "visits"],
        ),
        ("rpt.mv_abnormal_lab_daily", vec!["day", "abnormal_count", "total"]),
    ];
    let (table, cols) = allowed
        .iter()
        .find(|(t, _)| *t == body.table)
        .ok_or(actix_web::error::ErrorForbidden("table"))?;
    for c in &body.columns {
        if !cols.contains(&c.as_str()) {
            return Err(actix_web::error::ErrorForbidden("column"));
        }
    }
    let select = body.columns.join(",");
    let lim = body.limit.unwrap_or(500).clamp(1, 10_000);
    let sql = format!("SELECT {} FROM {} LIMIT $1", select, table);
    let rows = sqlx::query(&sql)
        .bind(lim)
        .fetch_all(&**db)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("db"))?;
    let mut out = Vec::new();
    for r in rows {
        let mut obj = serde_json::Map::new();
        for (i, c) in body.columns.iter().enumerate() {
            let v: Result<serde_json::Value, _> = r.try_get(i);
            if let Ok(val) = v {
                obj.insert(c.clone(), val);
            } else {
                obj.insert(c.clone(), serde_json::Value::Null);
            }
        }
        out.push(serde_json::Value::Object(obj));
    }
    Ok(HttpResponse::Ok().json(out))
}

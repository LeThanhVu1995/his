use chrono::{NaiveDate, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct RevenueDaily {
    pub day: chrono::DateTime<Utc>,
    pub revenue: f64,
    pub invoices: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct VisitsByDept {
    pub department_id: uuid::Uuid,
    pub department_name: Option<String>,
    pub day: chrono::DateTime<Utc>,
    pub visits: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AbnormalLabDaily {
    pub day: chrono::DateTime<Utc>,
    pub abnormal_count: i64,
    pub total: i64,
}

pub async fn revenue_range(
    db: &sqlx::Pool<sqlx::Postgres>,
    start: NaiveDate,
    end: NaiveDate,
) -> anyhow::Result<Vec<RevenueDaily>> {
    let rows = sqlx::query_as::<_, RevenueDaily>(
        r#"SELECT day, revenue, invoices FROM rpt.mv_revenue_daily WHERE day >= $1::date AND day < ($2::date + INTERVAL '1 day') ORDER BY day"#
    )
    .bind(start)
    .bind(end)
    .fetch_all(db)
    .await?;
    Ok(rows)
}

pub async fn visits_by_dept_range(
    db: &sqlx::Pool<sqlx::Postgres>,
    start: NaiveDate,
    end: NaiveDate,
) -> anyhow::Result<Vec<VisitsByDept>> {
    let rows = sqlx::query_as::<_, VisitsByDept>(
        r#"SELECT department_id, department_name, day, visits FROM rpt.mv_visits_by_dept WHERE day >= $1::date AND day < ($2::date + INTERVAL '1 day') ORDER BY day"#
    )
    .bind(start)
    .bind(end)
    .fetch_all(db)
    .await?;
    Ok(rows)
}

pub async fn abnormal_lab_range(
    db: &sqlx::Pool<sqlx::Postgres>,
    start: NaiveDate,
    end: NaiveDate,
) -> anyhow::Result<Vec<AbnormalLabDaily>> {
    let rows = sqlx::query_as::<_, AbnormalLabDaily>(
        r#"SELECT day, abnormal_count, total FROM rpt.mv_abnormal_lab_daily WHERE day >= $1::date AND day < ($2::date + INTERVAL '1 day') ORDER BY day"#
    )
    .bind(start)
    .bind(end)
    .fetch_all(db)
    .await?;
    Ok(rows)
}

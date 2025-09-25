use chrono::NaiveDate;
use serde_json::json;

pub struct ReportSvc<'a> {
    pub db: &'a sqlx::Pool<sqlx::Postgres>,
}

impl<'a> ReportSvc<'a> {
    pub async fn dashboard_overview(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> anyhow::Result<serde_json::Value> {
        let rev = crate::infra::db::views::revenue_range(self.db, start, end).await?;
        let vis = crate::infra::db::views::visits_by_dept_range(self.db, start, end).await?;
        let abn = crate::infra::db::views::abnormal_lab_range(self.db, start, end).await?;
        Ok(json!({
            "revenue_daily": rev,
            "visits_by_dept": vis,
            "abnormal_lab_daily": abn
        }))
    }
}

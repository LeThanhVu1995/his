use sqlx::{Pool, Postgres};
use crate::domain::entities::reconciliation::Reconciliation;

pub struct ReconRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> ReconRepo<'a> {
    pub async fn create(&self, r: &Reconciliation) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO reconciliations(id,batch_no,payer,period_start,period_end,total_claims,total_amount,approved_amount,created_at) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9)"#
        )
        .bind(r.id)
        .bind(&r.batch_no)
        .bind(&r.payer)
        .bind(r.period_start)
        .bind(r.period_end)
        .bind(r.total_claims)
        .bind(r.total_amount)
        .bind(r.approved_amount)
        .bind(r.created_at)
        .execute(self.db)
        .await?;
        Ok(())
    }
}

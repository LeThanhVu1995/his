use crate::infrastructure::repositories::recon_repo::ReconRepo;
use crate::domain::entities::reconciliation::Reconciliation;
use uuid::Uuid;

pub struct ReconSvc<'a> {
    pub repo: ReconRepo<'a>,
}

impl<'a> ReconSvc<'a> {
    pub async fn create_batch(
        &self,
        payer: &str,
        start: chrono::NaiveDate,
        end: chrono::NaiveDate,
        total: i64,
        amount: f64,
        approved: f64,
    ) -> anyhow::Result<Uuid> {
        let id = Uuid::new_v4();
        let r = Reconciliation {
            id,
            batch_no: format!("RCN-{}", &id.to_string()[..8]),
            payer: payer.into(),
            period_start: start,
            period_end: end,
            total_claims: total,
            total_amount: amount,
            approved_amount: approved,
            created_at: chrono::Utc::now(),
        };

        self.repo.create(&r).await?;
        Ok(id)
    }
}

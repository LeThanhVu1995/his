use anyhow::Result;
use crate::domain::entities::ins_policy::InsPolicy;
use crate::infrastructure::repositories::ins_policy_repo::InsPolicyRepo;
use sqlx::PgPool;
use chrono::NaiveDate;

pub struct InsPolicySvc<'a> {
    pub repo: InsPolicyRepo<'a>,
}

impl<'a> InsPolicySvc<'a> {
    pub fn new(db: &'a PgPool) -> Self {
        Self {
            repo: InsPolicyRepo::new(db),
        }
    }

    pub async fn create_policy(
        &self,
        patient_id: &str,
        payer_id: &str,
        policy_no: &str,
        coverage_json: Option<&str>,
        valid_from: Option<NaiveDate>,
        valid_to: Option<NaiveDate>,
    ) -> Result<InsPolicy> {
        let policy = InsPolicy {
            policy_id: uuid::Uuid::new_v4().to_string(),
            patient_id: patient_id.to_string(),
            payer_id: payer_id.to_string(),
            policy_no: policy_no.to_string(),
            coverage_json: coverage_json.map(|s| s.to_string()),
            valid_from,
            valid_to,
            status: "ACTIVE".to_string(),
        };

        self.repo.create(&policy).await?;
        Ok(policy)
    }

    pub async fn get_policy_by_id(&self, policy_id: &str) -> Result<Option<InsPolicy>> {
        self.repo.get_by_id(policy_id).await
    }

    pub async fn get_policy_by_policy_no(&self, payer_id: &str, policy_no: &str) -> Result<Option<InsPolicy>> {
        self.repo.get_by_policy_no(payer_id, policy_no).await
    }

    pub async fn list_policies_by_patient(&self, patient_id: &str, limit: i64, offset: i64) -> Result<Vec<InsPolicy>> {
        self.repo.list_by_patient(patient_id, limit, offset).await
    }

    pub async fn list_policies_by_payer(&self, payer_id: &str, limit: i64, offset: i64) -> Result<Vec<InsPolicy>> {
        self.repo.list_by_payer(payer_id, limit, offset).await
    }

    pub async fn list_policies_by_status(&self, status: &str, limit: i64, offset: i64) -> Result<Vec<InsPolicy>> {
        self.repo.list_by_status(status, limit, offset).await
    }

    pub async fn count_policies_by_patient(&self, patient_id: &str) -> Result<i64> {
        self.repo.count_by_patient(patient_id).await
    }

    pub async fn count_policies_by_payer(&self, payer_id: &str) -> Result<i64> {
        self.repo.count_by_payer(payer_id).await
    }

    pub async fn count_policies_by_status(&self, status: &str) -> Result<i64> {
        self.repo.count_by_status(status).await
    }

    pub async fn update_policy(&self, policy: &InsPolicy) -> Result<()> {
        self.repo.update(policy).await
    }

    pub async fn delete_policy(&self, policy_id: &str) -> Result<()> {
        self.repo.delete(policy_id).await
    }
}

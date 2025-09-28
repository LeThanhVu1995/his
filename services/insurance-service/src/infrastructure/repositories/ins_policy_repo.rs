use sqlx::PgPool;
use anyhow::Result;
use crate::domain::entities::ins_policy::InsPolicy;

pub struct InsPolicyRepo<'a> {
    pub db: &'a PgPool,
}

impl<'a> InsPolicyRepo<'a> {
    pub fn new(db: &'a PgPool) -> Self {
        Self { db }
    }

    pub async fn create(&self, policy: &InsPolicy) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO ins_policy (
                policy_id, patient_id, payer_id, policy_no,
                coverage_json, valid_from, valid_to, status
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#
        )
        .bind(&policy.policy_id)
        .bind(&policy.patient_id)
        .bind(&policy.payer_id)
        .bind(&policy.policy_no)
        .bind(&policy.coverage_json)
        .bind(&policy.valid_from)
        .bind(&policy.valid_to)
        .bind(&policy.status)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_by_id(&self, policy_id: &str) -> Result<Option<InsPolicy>> {
        let policy = sqlx::query_as::<_, InsPolicy>(
            r#"
            SELECT policy_id, patient_id, payer_id, policy_no,
                   coverage_json, valid_from, valid_to, status
            FROM ins_policy
            WHERE policy_id = $1
            "#
        )
        .bind(policy_id)
        .fetch_optional(self.db)
        .await?;
        Ok(policy)
    }

    pub async fn get_by_policy_no(&self, payer_id: &str, policy_no: &str) -> Result<Option<InsPolicy>> {
        let policy = sqlx::query_as::<_, InsPolicy>(
            r#"
            SELECT policy_id, patient_id, payer_id, policy_no,
                   coverage_json, valid_from, valid_to, status
            FROM ins_policy
            WHERE payer_id = $1 AND policy_no = $2
            "#
        )
        .bind(payer_id)
        .bind(policy_no)
        .fetch_optional(self.db)
        .await?;
        Ok(policy)
    }

    pub async fn list_by_patient(&self, patient_id: &str, limit: i64, offset: i64) -> Result<Vec<InsPolicy>> {
        let policies = sqlx::query_as::<_, InsPolicy>(
            r#"
            SELECT policy_id, patient_id, payer_id, policy_no,
                   coverage_json, valid_from, valid_to, status
            FROM ins_policy
            WHERE patient_id = $1
            ORDER BY valid_from DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(patient_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(policies)
    }

    pub async fn list_by_payer(&self, payer_id: &str, limit: i64, offset: i64) -> Result<Vec<InsPolicy>> {
        let policies = sqlx::query_as::<_, InsPolicy>(
            r#"
            SELECT policy_id, patient_id, payer_id, policy_no,
                   coverage_json, valid_from, valid_to, status
            FROM ins_policy
            WHERE payer_id = $1
            ORDER BY valid_from DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(payer_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(policies)
    }

    pub async fn list_by_status(&self, status: &str, limit: i64, offset: i64) -> Result<Vec<InsPolicy>> {
        let policies = sqlx::query_as::<_, InsPolicy>(
            r#"
            SELECT policy_id, patient_id, payer_id, policy_no,
                   coverage_json, valid_from, valid_to, status
            FROM ins_policy
            WHERE status = $1
            ORDER BY valid_from DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(status)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(policies)
    }

    pub async fn count_by_patient(&self, patient_id: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM ins_policy
            WHERE patient_id = $1
            "#
        )
        .bind(patient_id)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn count_by_payer(&self, payer_id: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM ins_policy
            WHERE payer_id = $1
            "#
        )
        .bind(payer_id)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn count_by_status(&self, status: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM ins_policy
            WHERE status = $1
            "#
        )
        .bind(status)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn update(&self, policy: &InsPolicy) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE ins_policy
            SET patient_id = $2, payer_id = $3, policy_no = $4,
                coverage_json = $5, valid_from = $6, valid_to = $7, status = $8
            WHERE policy_id = $1
            "#
        )
        .bind(&policy.policy_id)
        .bind(&policy.patient_id)
        .bind(&policy.payer_id)
        .bind(&policy.policy_no)
        .bind(&policy.coverage_json)
        .bind(&policy.valid_from)
        .bind(&policy.valid_to)
        .bind(&policy.status)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, policy_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM ins_policy
            WHERE policy_id = $1
            "#
        )
        .bind(policy_id)
        .execute(self.db)
        .await?;
        Ok(())
    }
}

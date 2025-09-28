use sqlx::PgPool;
use anyhow::Result;
use crate::domain::entities::claim::{InsClaim, InsClaimItem};

pub struct InsClaimRepo<'a> {
    pub db: &'a PgPool,
}

impl<'a> InsClaimRepo<'a> {
    pub fn new(db: &'a PgPool) -> Self {
        Self { db }
    }

    pub async fn create(&self, claim: &InsClaim) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO ins_claim (
                claim_id, encounter_id, policy_id, status,
                total_claimed, total_approved, submitted_at, response_at,
                response_code, response_text, signature_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#
        )
        .bind(&claim.claim_id)
        .bind(&claim.encounter_id)
        .bind(&claim.policy_id)
        .bind(&claim.status)
        .bind(&claim.total_claimed)
        .bind(&claim.total_approved)
        .bind(&claim.submitted_at)
        .bind(&claim.response_at)
        .bind(&claim.response_code)
        .bind(&claim.response_text)
        .bind(&claim.signature_id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_by_id(&self, claim_id: &str) -> Result<Option<InsClaim>> {
        let claim = sqlx::query_as::<_, InsClaim>(
            r#"
            SELECT claim_id, encounter_id, policy_id, status,
                   total_claimed, total_approved, submitted_at, response_at,
                   response_code, response_text, signature_id
            FROM ins_claim
            WHERE claim_id = $1
            "#
        )
        .bind(claim_id)
        .fetch_optional(self.db)
        .await?;
        Ok(claim)
    }

    pub async fn list_by_encounter(&self, encounter_id: &str, limit: i64, offset: i64) -> Result<Vec<InsClaim>> {
        let claims = sqlx::query_as::<_, InsClaim>(
            r#"
            SELECT claim_id, encounter_id, policy_id, status,
                   total_claimed, total_approved, submitted_at, response_at,
                   response_code, response_text, signature_id
            FROM ins_claim
            WHERE encounter_id = $1
            ORDER BY submitted_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(encounter_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(claims)
    }

    pub async fn list_by_policy(&self, policy_id: &str, limit: i64, offset: i64) -> Result<Vec<InsClaim>> {
        let claims = sqlx::query_as::<_, InsClaim>(
            r#"
            SELECT claim_id, encounter_id, policy_id, status,
                   total_claimed, total_approved, submitted_at, response_at,
                   response_code, response_text, signature_id
            FROM ins_claim
            WHERE policy_id = $1
            ORDER BY submitted_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(policy_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(claims)
    }

    pub async fn list_by_status(&self, status: &str, limit: i64, offset: i64) -> Result<Vec<InsClaim>> {
        let claims = sqlx::query_as::<_, InsClaim>(
            r#"
            SELECT claim_id, encounter_id, policy_id, status,
                   total_claimed, total_approved, submitted_at, response_at,
                   response_code, response_text, signature_id
            FROM ins_claim
            WHERE status = $1
            ORDER BY submitted_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(status)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(claims)
    }

    pub async fn count_by_encounter(&self, encounter_id: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM ins_claim
            WHERE encounter_id = $1
            "#
        )
        .bind(encounter_id)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn count_by_policy(&self, policy_id: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM ins_claim
            WHERE policy_id = $1
            "#
        )
        .bind(policy_id)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn count_by_status(&self, status: &str) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM ins_claim
            WHERE status = $1
            "#
        )
        .bind(status)
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn update(&self, claim: &InsClaim) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE ins_claim
            SET encounter_id = $2, policy_id = $3, status = $4,
                total_claimed = $5, total_approved = $6, submitted_at = $7,
                response_at = $8, response_code = $9, response_text = $10,
                signature_id = $11
            WHERE claim_id = $1
            "#
        )
        .bind(&claim.claim_id)
        .bind(&claim.encounter_id)
        .bind(&claim.policy_id)
        .bind(&claim.status)
        .bind(&claim.total_claimed)
        .bind(&claim.total_approved)
        .bind(&claim.submitted_at)
        .bind(&claim.response_at)
        .bind(&claim.response_code)
        .bind(&claim.response_text)
        .bind(&claim.signature_id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, claim_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM ins_claim
            WHERE claim_id = $1
            "#
        )
        .bind(claim_id)
        .execute(self.db)
        .await?;
        Ok(())
    }

    // Claim Item methods
    pub async fn create_claim_item(&self, item: &InsClaimItem) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO ins_claim_item (
                claim_item_id, claim_id, service_code, description,
                qty, unit_price, amount, approved_amount
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#
        )
        .bind(&item.claim_item_id)
        .bind(&item.claim_id)
        .bind(&item.service_code)
        .bind(&item.description)
        .bind(&item.qty)
        .bind(&item.unit_price)
        .bind(&item.amount)
        .bind(&item.approved_amount)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_claim_items(&self, claim_id: &str) -> Result<Vec<InsClaimItem>> {
        let items = sqlx::query_as::<_, InsClaimItem>(
            r#"
            SELECT claim_item_id, claim_id, service_code, description,
                   qty, unit_price, amount, approved_amount
            FROM ins_claim_item
            WHERE claim_id = $1
            ORDER BY service_code
            "#
        )
        .bind(claim_id)
        .fetch_all(self.db)
        .await?;
        Ok(items)
    }

    pub async fn update_claim_item(&self, item: &InsClaimItem) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE ins_claim_item
            SET claim_id = $2, service_code = $3, description = $4,
                qty = $5, unit_price = $6, amount = $7, approved_amount = $8
            WHERE claim_item_id = $1
            "#
        )
        .bind(&item.claim_item_id)
        .bind(&item.claim_id)
        .bind(&item.service_code)
        .bind(&item.description)
        .bind(&item.qty)
        .bind(&item.unit_price)
        .bind(&item.amount)
        .bind(&item.approved_amount)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn delete_claim_item(&self, claim_item_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM ins_claim_item
            WHERE claim_item_id = $1
            "#
        )
        .bind(claim_item_id)
        .execute(self.db)
        .await?;
        Ok(())
    }
}

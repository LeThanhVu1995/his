use uuid::Uuid;
use sqlx::{Pool, Postgres};
use crate::domain::entities::claim::{InsClaim, InsClaimItem};

pub struct ClaimRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> ClaimRepo<'a> {
    pub async fn create(&self, c: &InsClaim) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO ins_claim(claim_id,encounter_id,policy_id,status,total_claimed) VALUES($1,$2,$3,$4,$5)"#
        )
        .bind(&c.claim_id)
        .bind(&c.encounter_id)
        .bind(&c.policy_id)
        .bind(&c.status)
        .bind(&c.total_claimed)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn add_item(&self, it: &InsClaimItem) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO ins_claim_item(claim_item_id,claim_id,service_code,description,qty,unit_price,amount) VALUES($1,$2,$3,$4,$5,$6,$7)"#
        )
        .bind(&it.claim_item_id)
        .bind(&it.claim_id)
        .bind(&it.service_code)
        .bind(&it.description)
        .bind(&it.qty)
        .bind(&it.unit_price)
        .bind(&it.amount)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get(&self, id: Uuid) -> anyhow::Result<Option<InsClaim>> {
        Ok(sqlx::query_as::<_, InsClaim>(
            r#"SELECT claim_id,encounter_id,policy_id,status,total_claimed,total_approved,submitted_at,response_at,response_code,response_text,signature_id FROM ins_claim WHERE claim_id=$1"#
        )
        .bind(id.to_string())
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn update_status(&self, id: Uuid, st: &str) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE ins_claim SET status=$2 WHERE claim_id=$1"#
        )
        .bind(id.to_string())
        .bind(st)
        .execute(self.db)
        .await?;
        Ok(())
    }
}

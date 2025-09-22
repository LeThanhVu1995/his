use uuid::Uuid;
use sqlx::{Pool, Postgres};
use crate::domain::entities::claim::{Claim, ClaimItem};

pub struct ClaimRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> ClaimRepo<'a> {
    pub async fn create(&self, c: &Claim) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO claims(id,claim_no,patient_id,encounter_id,member_id,payer,total_amount,currency,status) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9)"#
        )
        .bind(c.id)
        .bind(&c.claim_no)
        .bind(c.patient_id)
        .bind(c.encounter_id)
        .bind(c.member_id)
        .bind(&c.payer)
        .bind(c.total_amount)
        .bind(&c.currency)
        .bind(&c.status)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn add_item(&self, it: &ClaimItem) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO claim_items(id,claim_id,code,description,qty,unit_price,coverage_rate,patient_pay) VALUES($1,$2,$3,$4,$5,$6,$7,$8)"#
        )
        .bind(it.id)
        .bind(it.claim_id)
        .bind(&it.code)
        .bind(&it.description)
        .bind(it.qty)
        .bind(it.unit_price)
        .bind(it.coverage_rate)
        .bind(it.patient_pay)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get(&self, id: Uuid) -> anyhow::Result<Option<Claim>> {
        Ok(sqlx::query_as::<_, Claim>(
            r#"SELECT id,claim_no,patient_id,encounter_id,member_id,payer,total_amount,currency,status,created_at,updated_at FROM claims WHERE id=$1"#
        )
        .bind(id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn update_status(&self, id: Uuid, st: &str) -> anyhow::Result<()> {
        sqlx::query(
            r#"UPDATE claims SET status=$2, updated_at=NOW() WHERE id=$1"#
        )
        .bind(id)
        .bind(st)
        .execute(self.db)
        .await?;
        Ok(())
    }
}

use sqlx::PgPool;
use anyhow::Result;
use crate::domain::entities::ins_payer::InsPayer;

pub struct InsPayerRepo<'a> {
    pub db: &'a PgPool,
}

impl<'a> InsPayerRepo<'a> {
    pub fn new(db: &'a PgPool) -> Self {
        Self { db }
    }

    pub async fn create(&self, payer: &InsPayer) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO ins_payer (payer_id, code, name)
            VALUES ($1, $2, $3)
            "#
        )
        .bind(&payer.payer_id)
        .bind(&payer.code)
        .bind(&payer.name)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_by_id(&self, payer_id: &str) -> Result<Option<InsPayer>> {
        let payer = sqlx::query_as::<_, InsPayer>(
            r#"
            SELECT payer_id, code, name
            FROM ins_payer
            WHERE payer_id = $1
            "#
        )
        .bind(payer_id)
        .fetch_optional(self.db)
        .await?;
        Ok(payer)
    }

    pub async fn get_by_code(&self, code: &str) -> Result<Option<InsPayer>> {
        let payer = sqlx::query_as::<_, InsPayer>(
            r#"
            SELECT payer_id, code, name
            FROM ins_payer
            WHERE code = $1
            "#
        )
        .bind(code)
        .fetch_optional(self.db)
        .await?;
        Ok(payer)
    }

    pub async fn list(&self, limit: i64, offset: i64) -> Result<Vec<InsPayer>> {
        let payers = sqlx::query_as::<_, InsPayer>(
            r#"
            SELECT payer_id, code, name
            FROM ins_payer
            ORDER BY name
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?;
        Ok(payers)
    }

    pub async fn count(&self) -> Result<i64> {
        let count = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM ins_payer
            "#
        )
        .fetch_one(self.db)
        .await?;
        Ok(count)
    }

    pub async fn update(&self, payer: &InsPayer) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE ins_payer
            SET code = $2, name = $3
            WHERE payer_id = $1
            "#
        )
        .bind(&payer.payer_id)
        .bind(&payer.code)
        .bind(&payer.name)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, payer_id: &str) -> Result<()> {
        sqlx::query(
            r#"
            DELETE FROM ins_payer
            WHERE payer_id = $1
            "#
        )
        .bind(payer_id)
        .execute(self.db)
        .await?;
        Ok(())
    }
}

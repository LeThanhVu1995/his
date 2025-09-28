use uuid::Uuid;
use sqlx::{Pool, Postgres};
use crate::domain::entities::crossmatch::Crossmatch;

pub struct CrossmatchRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> CrossmatchRepo<'a> {
    pub async fn insert(&self, c: &Crossmatch) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO bb_crossmatch(crossmatch_id, patient_id, unit_id, performed_at, result_code, performer_id, created_at, updated_at)
             VALUES($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(c.crossmatch_id)
        .bind(c.patient_id)
        .bind(c.unit_id)
        .bind(c.performed_at)
        .bind(&c.result_code)
        .bind(c.performer_id)
        .bind(c.created_at)
        .bind(c.updated_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn list_by_patient(&self, patient_id: Uuid) -> anyhow::Result<Vec<Crossmatch>> {
        Ok(sqlx::query_as::<_, Crossmatch>(
            "SELECT crossmatch_id, patient_id, unit_id, performed_at, result_code, performer_id, created_at, updated_at
             FROM bb_crossmatch WHERE patient_id = $1 ORDER BY performed_at DESC"
        )
        .bind(patient_id)
        .fetch_all(self.db)
        .await?)
    }
}

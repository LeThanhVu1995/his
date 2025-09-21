use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::domain::models::PrescriptionItem;

pub struct PrescriptionItemRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> PrescriptionItemRepo<'a> {
    pub async fn insert_many(&self, items: &[PrescriptionItem]) -> anyhow::Result<()> {
        for it in items {
            sqlx::query(
                r#"INSERT INTO prescription_items(id,prescription_id,medication_id,dose,freq,duration,qty,instruction) VALUES($1,$2,$3,$4,$5,$6,$7,$8)"#
            )
            .bind(it.id)
            .bind(it.prescription_id)
            .bind(it.medication_id)
            .bind(&it.dose)
            .bind(&it.freq)
            .bind(&it.duration)
            .bind(it.qty)
            .bind(&it.instruction)
            .execute(self.db)
            .await?;
        }
        Ok(())
    }

    pub async fn list_by_prescription(&self, pid: Uuid) -> anyhow::Result<Vec<PrescriptionItem>> {
        Ok(sqlx::query_as::<_, PrescriptionItem>(
            r#"SELECT id,prescription_id,medication_id,dose,freq,duration,qty,instruction,created_at,updated_at FROM prescription_items WHERE prescription_id=$1 ORDER BY created_at"#
        )
        .bind(pid)
        .fetch_all(self.db)
        .await?)
    }
}

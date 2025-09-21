use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::domain::models::Dispense;

pub struct DispenseRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> DispenseRepo<'a> {
    pub async fn create(&self, d: &Dispense) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO dispenses(id,prescription_id,disp_no,dispensed_by,status) VALUES($1,$2,$3,$4,$5)"#
        )
        .bind(d.id)
        .bind(d.prescription_id)
        .bind(&d.disp_no)
        .bind(&d.dispensed_by)
        .bind(&d.status)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn finish(&self, id: Uuid) -> anyhow::Result<Option<Dispense>> {
        let rec = sqlx::query_as::<_, Dispense>(
            r#"UPDATE dispenses SET status='COMPLETED', updated_at=NOW() WHERE id=$1 RETURNING id,prescription_id,disp_no,dispensed_by,status,created_at,updated_at"#
        )
        .bind(id)
        .fetch_optional(self.db)
        .await?;
        Ok(rec)
    }

    pub async fn list_paged(&self, prescription_id: Option<Uuid>, page: i64, size: i64) -> anyhow::Result<(Vec<Dispense>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;

        let (rows, total) = match prescription_id {
            Some(pid) => {
                let r = sqlx::query_as::<_, Dispense>(
                    r#"SELECT id,prescription_id,disp_no,dispensed_by,status,created_at,updated_at FROM dispenses WHERE prescription_id=$1 ORDER BY created_at DESC OFFSET $2 LIMIT $3"#
                )
                .bind(pid)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM dispenses WHERE prescription_id=$1")
                    .bind(pid)
                    .fetch_one(self.db)
                    .await?;
                (r, t)
            },
            None => {
                let r = sqlx::query_as::<_, Dispense>(
                    r#"SELECT id,prescription_id,disp_no,dispensed_by,status,created_at,updated_at FROM dispenses ORDER BY created_at DESC OFFSET $1 LIMIT $2"#
                )
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM dispenses")
                    .fetch_one(self.db)
                    .await?;
                (r, t)
            }
        };
        Ok((rows, total))
    }

    pub async fn find(&self, id: Uuid) -> anyhow::Result<Option<Dispense>> {
        Ok(sqlx::query_as::<_, Dispense>(
            r#"SELECT id,prescription_id,disp_no,dispensed_by,status,created_at,updated_at FROM dispenses WHERE id=$1"#
        )
        .bind(id)
        .fetch_optional(self.db)
        .await?)
    }
}

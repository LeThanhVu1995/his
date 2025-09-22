use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::domain::models::LabTest;

pub struct TestRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> TestRepo<'a> {
    pub async fn create(&self, t: &LabTest) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO lab_tests(id,code,name,specimen_type,unit,ref_low,ref_high) VALUES($1,$2,$3,$4,$5,$6,$7)"#
        )
        .bind(t.id)
        .bind(&t.code)
        .bind(&t.name)
        .bind(&t.specimen_type)
        .bind(&t.unit)
        .bind(&t.ref_low)
        .bind(&t.ref_high)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn update(&self, id: Uuid, name: Option<&str>, unit: Option<&str>, ref_low: Option<f64>, ref_high: Option<f64>) -> anyhow::Result<Option<LabTest>> {
        let rec = sqlx::query_as::<_, LabTest>(
            r#"UPDATE lab_tests SET name=COALESCE($2,name), unit=COALESCE($3,unit), ref_low=COALESCE($4::NUMERIC,ref_low), ref_high=COALESCE($5::NUMERIC,ref_high), updated_at=NOW() WHERE id=$1 RETURNING id,code,name,specimen_type,unit,ref_low,ref_high,created_at,updated_at"#
        )
        .bind(id)
        .bind(name)
        .bind(unit)
        .bind(ref_low)
        .bind(ref_high)
        .fetch_optional(self.db)
        .await?;
        Ok(rec)
    }

    pub async fn search_paged(&self, q: Option<&str>, page: i64, size: i64) -> anyhow::Result<(Vec<LabTest>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;

        let (rows, total) = if let Some(q) = q {
            let like = format!("%{}%", q);
            let r = sqlx::query_as::<_, LabTest>(
                r#"SELECT id,code,name,specimen_type,unit,ref_low,ref_high,created_at,updated_at FROM lab_tests WHERE code ILIKE $1 OR name ILIKE $1 ORDER BY code OFFSET $2 LIMIT $3"#
            )
            .bind(&like)
            .bind(offset)
            .bind(size)
            .fetch_all(self.db)
            .await?;
            let t = sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM lab_tests WHERE code ILIKE $1 OR name ILIKE $1")
                .bind(&like)
                .fetch_one(self.db)
                .await?;
            (r, t)
        } else {
            let r = sqlx::query_as::<_, LabTest>(
                r#"SELECT id,code,name,specimen_type,unit,ref_low,ref_high,created_at,updated_at FROM lab_tests ORDER BY code OFFSET $1 LIMIT $2"#
            )
            .bind(offset)
            .bind(size)
            .fetch_all(self.db)
            .await?;
            let t = sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM lab_tests")
                .fetch_one(self.db)
                .await?;
            (r, t)
        };
        Ok((rows, total))
    }

    pub async fn find(&self, id: Uuid) -> anyhow::Result<Option<LabTest>> {
        Ok(sqlx::query_as::<_, LabTest>(
            r#"SELECT id,code,name,specimen_type,unit,ref_low,ref_high,created_at,updated_at FROM lab_tests WHERE id=$1"#
        )
        .bind(id)
        .fetch_optional(self.db)
        .await?)
    }
}

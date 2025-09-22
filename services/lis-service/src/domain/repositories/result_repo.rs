use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::domain::models::{LabResult, ResultValue};

pub struct ResultRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> ResultRepo<'a> {
    pub async fn create(&self, r: &LabResult) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO lab_results(id,result_no,specimen_id,test_id,status,verified_by,verified_at,released_by,released_at,note) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)"#
        )
        .bind(r.id)
        .bind(&r.result_no)
        .bind(r.specimen_id)
        .bind(r.test_id)
        .bind(&r.status)
        .bind(&r.verified_by)
        .bind(&r.verified_at)
        .bind(&r.released_by)
        .bind(&r.released_at)
        .bind(&r.note)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn list_paged(&self, specimen_id: Option<Uuid>, status: Option<&str>, page: i64, size: i64) -> anyhow::Result<(Vec<LabResult>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;

        let (rows, total) = match (specimen_id, status) {
            (Some(s), Some(st)) => {
                let r = sqlx::query_as::<_, LabResult>(
                    r#"SELECT id,result_no,specimen_id,test_id,status,verified_by,verified_at,released_by,released_at,note,created_at,updated_at FROM lab_results WHERE specimen_id=$1 AND status=$2 ORDER BY created_at DESC OFFSET $3 LIMIT $4"#
                )
                .bind(s)
                .bind(st)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM lab_results WHERE specimen_id=$1 AND status=$2")
                    .bind(s)
                    .bind(st)
                    .fetch_one(self.db)
                    .await?;
                (r, t)
            },
            _ => {
                let r = sqlx::query_as::<_, LabResult>(
                    r#"SELECT id,result_no,specimen_id,test_id,status,verified_by,verified_at,released_by,released_at,note,created_at,updated_at FROM lab_results ORDER BY created_at DESC OFFSET $1 LIMIT $2"#
                )
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM lab_results")
                    .fetch_one(self.db)
                    .await?;
                (r, t)
            }
        };
        Ok((rows, total))
    }

    pub async fn enter_values(&self, rid: Uuid, items: &[ResultValue]) -> anyhow::Result<()> {
        for v in items {
            sqlx::query(
                r#"INSERT INTO lab_result_values(id,result_id,analyte_code,analyte_name,value_num,value_text,unit,ref_low,ref_high,flag) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)"#
            )
            .bind(v.id)
            .bind(v.result_id)
            .bind(&v.analyte_code)
            .bind(&v.analyte_name)
            .bind(&v.value_num)
            .bind(&v.value_text)
            .bind(&v.unit)
            .bind(&v.ref_low)
            .bind(&v.ref_high)
            .bind(&v.flag)
            .execute(self.db)
            .await?;
        }
        Ok(())
    }

    pub async fn update_status(&self, id: Uuid, status: &str, by: Option<&str>) -> anyhow::Result<Option<LabResult>> {
        let (verified_by, released_by) = match status {
            "VERIFIED" => (by, None),
            "RELEASED" => (None, by),
            _ => (None, None),
        };
        let rec = sqlx::query_as::<_, LabResult>(
            r#"UPDATE lab_results SET status=$2, verified_by=COALESCE($3,verified_by), verified_at=CASE WHEN $2='VERIFIED' THEN NOW() ELSE verified_at END, released_by=COALESCE($4,released_by), released_at=CASE WHEN $2='RELEASED' THEN NOW() ELSE released_at END, updated_at=NOW() WHERE id=$1 RETURNING id,result_no,specimen_id,test_id,status,verified_by,verified_at,released_by,released_at,note,created_at,updated_at"#
        )
        .bind(id)
        .bind(status)
        .bind(verified_by)
        .bind(released_by)
        .fetch_optional(self.db)
        .await?;
        Ok(rec)
    }

    pub async fn find(&self, id: Uuid) -> anyhow::Result<Option<LabResult>> {
        Ok(sqlx::query_as::<_, LabResult>(
            r#"SELECT id,result_no,specimen_id,test_id,status,verified_by,verified_at,released_by,released_at,note,created_at,updated_at FROM lab_results WHERE id=$1"#
        )
        .bind(id)
        .fetch_optional(self.db)
        .await?)
    }
}

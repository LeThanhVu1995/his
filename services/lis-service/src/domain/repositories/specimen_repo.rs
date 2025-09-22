use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::domain::models::Specimen;

pub struct SpecimenRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> SpecimenRepo<'a> {
    pub async fn create(&self, s: &Specimen) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO lab_specimens(id,specimen_no,order_id,patient_id,encounter_id,specimen_type,collected_at,collected_by,status,note) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)"#
        )
        .bind(s.id)
        .bind(&s.specimen_no)
        .bind(&s.order_id)
        .bind(s.patient_id)
        .bind(&s.encounter_id)
        .bind(&s.specimen_type)
        .bind(&s.collected_at)
        .bind(&s.collected_by)
        .bind(&s.status)
        .bind(&s.note)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn update_status(&self, id: Uuid, status: &str, collected_at: Option<chrono::DateTime<chrono::Utc>>, collected_by: Option<&str>) -> anyhow::Result<Option<Specimen>> {
        let rec = sqlx::query_as::<_, Specimen>(
            r#"UPDATE lab_specimens SET status=$2, collected_at=COALESCE($3,collected_at), collected_by=COALESCE($4,collected_by), updated_at=NOW() WHERE id=$1 RETURNING id,specimen_no,order_id,patient_id,encounter_id,specimen_type,collected_at,collected_by,status,note,created_at,updated_at"#
        )
        .bind(id)
        .bind(status)
        .bind(collected_at)
        .bind(collected_by)
        .fetch_optional(self.db)
        .await?;
        Ok(rec)
    }

    pub async fn list_paged(&self, patient_id: Option<Uuid>, status: Option<&str>, page: i64, size: i64) -> anyhow::Result<(Vec<Specimen>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;

        let (rows, total) = match (patient_id, status) {
            (Some(p), Some(s)) => {
                let r = sqlx::query_as::<_, Specimen>(
                    r#"SELECT id,specimen_no,order_id,patient_id,encounter_id,specimen_type,collected_at,collected_by,status,note,created_at,updated_at FROM lab_specimens WHERE patient_id=$1 AND status=$2 ORDER BY created_at DESC OFFSET $3 LIMIT $4"#
                )
                .bind(p)
                .bind(s)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM lab_specimens WHERE patient_id=$1 AND status=$2")
                    .bind(p)
                    .bind(s)
                    .fetch_one(self.db)
                    .await?;
                (r, t)
            },
            (Some(p), None) => {
                let r = sqlx::query_as::<_, Specimen>(
                    r#"SELECT id,specimen_no,order_id,patient_id,encounter_id,specimen_type,collected_at,collected_by,status,note,created_at,updated_at FROM lab_specimens WHERE patient_id=$1 ORDER BY created_at DESC OFFSET $2 LIMIT $3"#
                )
                .bind(p)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM lab_specimens WHERE patient_id=$1")
                    .bind(p)
                    .fetch_one(self.db)
                    .await?;
                (r, t)
            },
            _ => {
                let r = sqlx::query_as::<_, Specimen>(
                    r#"SELECT id,specimen_no,order_id,patient_id,encounter_id,specimen_type,collected_at,collected_by,status,note,created_at,updated_at FROM lab_specimens ORDER BY created_at DESC OFFSET $1 LIMIT $2"#
                )
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM lab_specimens")
                    .fetch_one(self.db)
                    .await?;
                (r, t)
            }
        };
        Ok((rows, total))
    }

    pub async fn find(&self, id: Uuid) -> anyhow::Result<Option<Specimen>> {
        Ok(sqlx::query_as::<_, Specimen>(
            r#"SELECT id,specimen_no,order_id,patient_id,encounter_id,specimen_type,collected_at,collected_by,status,note,created_at,updated_at FROM lab_specimens WHERE id=$1"#
        )
        .bind(id)
        .fetch_optional(self.db)
        .await?)
    }
}

use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::domain::models::Prescription;

pub struct PrescriptionRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> PrescriptionRepo<'a> {
    pub async fn create(&self, p: &Prescription) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO prescriptions(id,patient_id,encounter_id,presc_no,status,ordered_by,note) VALUES($1,$2,$3,$4,$5,$6,$7)"#
        )
        .bind(p.id)
        .bind(p.patient_id)
        .bind(&p.encounter_id)
        .bind(&p.presc_no)
        .bind(&p.status)
        .bind(&p.ordered_by)
        .bind(&p.note)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn update(&self, id: Uuid, status: Option<&str>, note: Option<&str>) -> anyhow::Result<Option<Prescription>> {
        let rec = sqlx::query_as::<_, Prescription>(
            r#"UPDATE prescriptions SET status=COALESCE($2,status), note=COALESCE($3,note), updated_at=NOW() WHERE id=$1 RETURNING id,patient_id,encounter_id,presc_no,status,ordered_by,note,created_at,updated_at"#
        )
        .bind(id)
        .bind(status)
        .bind(note)
        .fetch_optional(self.db)
        .await?;
        Ok(rec)
    }

    pub async fn list_paged(&self, patient_id: Option<Uuid>, status: Option<&str>, page: i64, size: i64) -> anyhow::Result<(Vec<Prescription>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;

        let (rows, total) = match (patient_id, status) {
            (Some(p), Some(s)) => {
                let r = sqlx::query_as::<_, Prescription>(
                    r#"SELECT id,patient_id,encounter_id,presc_no,status,ordered_by,note,created_at,updated_at FROM prescriptions WHERE patient_id=$1 AND status=$2 ORDER BY created_at DESC OFFSET $3 LIMIT $4"#
                )
                .bind(p)
                .bind(s)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM prescriptions WHERE patient_id=$1 AND status=$2")
                    .bind(p)
                    .bind(s)
                    .fetch_one(self.db)
                    .await?;
                (r, t)
            },
            (Some(p), None) => {
                let r = sqlx::query_as::<_, Prescription>(
                    r#"SELECT id,patient_id,encounter_id,presc_no,status,ordered_by,note,created_at,updated_at FROM prescriptions WHERE patient_id=$1 ORDER BY created_at DESC OFFSET $2 LIMIT $3"#
                )
                .bind(p)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM prescriptions WHERE patient_id=$1")
                    .bind(p)
                    .fetch_one(self.db)
                    .await?;
                (r, t)
            },
            _ => {
                let r = sqlx::query_as::<_, Prescription>(
                    r#"SELECT id,patient_id,encounter_id,presc_no,status,ordered_by,note,created_at,updated_at FROM prescriptions ORDER BY created_at DESC OFFSET $1 LIMIT $2"#
                )
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM prescriptions")
                    .fetch_one(self.db)
                    .await?;
                (r, t)
            }
        };
        Ok((rows, total))
    }

    pub async fn find(&self, id: Uuid) -> anyhow::Result<Option<Prescription>> {
        Ok(sqlx::query_as::<_, Prescription>(
            r#"SELECT id,patient_id,encounter_id,presc_no,status,ordered_by,note,created_at,updated_at FROM prescriptions WHERE id=$1"#
        )
        .bind(id)
        .fetch_optional(self.db)
        .await?)
    }
}

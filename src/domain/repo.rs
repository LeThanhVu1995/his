use sqlx::{Pool, Postgres};
use uuid::Uuid;
use rust_decimal::Decimal;
use crate::domain::models::{Medication, Prescription, PrescriptionItem, Dispense};

pub struct MedRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> MedRepo<'a> {
    pub async fn create(&self, m: &Medication) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO medications(id,code,name,strength,form,route) VALUES($1,$2,$3,$4,$5,$6)"#,
            m.id, m.code, m.name, m.strength, m.form, m.route
        )
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn update(
        &self,
        id: Uuid,
        name: Option<&str>,
        strength: Option<&str>,
        form: Option<&str>,
        route: Option<&str>,
    ) -> anyhow::Result<Option<Medication>> {
        let rec = sqlx::query_as!(
            Medication,
            r#"UPDATE medications SET name=COALESCE($2,name), strength=COALESCE($3,strength), form=COALESCE($4,form), route=COALESCE($5,route), updated_at=NOW() WHERE id=$1 RETURNING id,code,name,strength,form,route,created_at,updated_at"#,
            id, name, strength, form, route
        )
        .fetch_optional(self.db)
        .await?;
        Ok(rec)
    }

    pub async fn search_paged(
        &self,
        q: Option<&str>,
        page: i64,
        size: i64,
    ) -> anyhow::Result<(Vec<Medication>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;
        let (rows, total) = if let Some(q) = q {
            let like = format!("%{}%", q);
            let r = sqlx::query_as!(
                Medication,
                r#"SELECT id,code,name,strength,form,route,created_at,updated_at FROM medications WHERE code ILIKE $1 OR name ILIKE $1 ORDER BY name OFFSET $2 LIMIT $3"#,
                like, offset, size
            )
            .fetch_all(self.db)
            .await?;
            let t = sqlx::query_scalar!(
                "SELECT COUNT(1) FROM medications WHERE code ILIKE $1 OR name ILIKE $1",
                like
            )
            .fetch_one(self.db)
            .await?;
            (r, t)
        } else {
            let r = sqlx::query_as!(
                Medication,
                r#"SELECT id,code,name,strength,form,route,created_at,updated_at FROM medications ORDER BY name OFFSET $1 LIMIT $2"#,
                offset, size
            )
            .fetch_all(self.db)
            .await?;
            let t = sqlx::query_scalar!("SELECT COUNT(1) FROM medications")
                .fetch_one(self.db)
                .await?;
            (r, t)
        };
        Ok((rows, total))
    }

    pub async fn find(&self, id: Uuid) -> anyhow::Result<Option<Medication>> {
        Ok(sqlx::query_as!(
            Medication,
            r#"SELECT id,code,name,strength,form,route,created_at,updated_at FROM medications WHERE id=$1"#,
            id
        )
        .fetch_optional(self.db)
        .await?)
    }
}

pub struct PrescRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> PrescRepo<'a> {
    pub async fn create(&self, p: &Prescription) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO prescriptions(id,patient_id,encounter_id,presc_no,status,ordered_by,note) VALUES($1,$2,$3,$4,$5,$6,$7)"#,
            p.id, p.patient_id, p.encounter_id, p.presc_no, p.status, p.ordered_by, p.note
        )
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn update(
        &self,
        id: Uuid,
        status: Option<&str>,
        note: Option<&str>,
    ) -> anyhow::Result<Option<Prescription>> {
        let rec = sqlx::query_as!(
            Prescription,
            r#"UPDATE prescriptions SET status=COALESCE($2,status), note=COALESCE($3,note), updated_at=NOW() WHERE id=$1 RETURNING id,patient_id,encounter_id,presc_no,status,ordered_by,note,created_at,updated_at"#,
            id, status, note
        )
        .fetch_optional(self.db)
        .await?;
        Ok(rec)
    }

    pub async fn list_paged(
        &self,
        patient_id: Option<Uuid>,
        status: Option<&str>,
        page: i64,
        size: i64,
    ) -> anyhow::Result<(Vec<Prescription>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;
        let (rows, total) = match (patient_id, status) {
            (Some(p), Some(s)) => {
                let r = sqlx::query_as!(
                    Prescription,
                    r#"SELECT id,patient_id,encounter_id,presc_no,status,ordered_by,note,created_at,updated_at FROM prescriptions WHERE patient_id=$1 AND status=$2 ORDER BY created_at DESC OFFSET $3 LIMIT $4"#,
                    p, s, offset, size
                )
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar!(
                    "SELECT COUNT(1) FROM prescriptions WHERE patient_id=$1 AND status=$2",
                    p, s
                )
                .fetch_one(self.db)
                .await?;
                (r, t)
            }
            (Some(p), None) => {
                let r = sqlx::query_as!(
                    Prescription,
                    r#"SELECT id,patient_id,encounter_id,presc_no,status,ordered_by,note,created_at,updated_at FROM prescriptions WHERE patient_id=$1 ORDER BY created_at DESC OFFSET $2 LIMIT $3"#,
                    p, offset, size
                )
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar!(
                    "SELECT COUNT(1) FROM prescriptions WHERE patient_id=$1",
                    p
                )
                .fetch_one(self.db)
                .await?;
                (r, t)
            }
            _ => {
                let r = sqlx::query_as!(
                    Prescription,
                    r#"SELECT id,patient_id,encounter_id,presc_no,status,ordered_by,note,created_at,updated_at FROM prescriptions ORDER BY created_at DESC OFFSET $1 LIMIT $2"#,
                    offset, size
                )
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar!("SELECT COUNT(1) FROM prescriptions")
                    .fetch_one(self.db)
                    .await?;
                (r, t)
            }
        };
        Ok((rows, total))
    }

    pub async fn find(&self, id: Uuid) -> anyhow::Result<Option<Prescription>> {
        Ok(sqlx::query_as!(
            Prescription,
            r#"SELECT id,patient_id,encounter_id,presc_no,status,ordered_by,note,created_at,updated_at FROM prescriptions WHERE id=$1"#,
            id
        )
        .fetch_optional(self.db)
        .await?)
    }
}

pub struct PrescItemRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> PrescItemRepo<'a> {
    pub async fn insert_many(&self, items: &[PrescriptionItem]) -> anyhow::Result<()> {
        for it in items {
            sqlx::query!(
                r#"INSERT INTO prescription_items(id,prescription_id,medication_id,dose,freq,duration,qty,instruction) VALUES($1,$2,$3,$4,$5,$6,$7,$8)"#,
                it.id, it.prescription_id, it.medication_id, it.dose, it.freq, it.duration, it.qty.to_string(), it.instruction
            )
            .execute(self.db)
            .await?;
        }
        Ok(())
    }

    pub async fn list_by_prescription(&self, pid: Uuid) -> anyhow::Result<Vec<PrescriptionItem>> {
        Ok(sqlx::query_as!(
            PrescriptionItem,
            r#"SELECT id,prescription_id,medication_id,dose,freq,duration,qty,instruction,created_at,updated_at FROM prescription_items WHERE prescription_id=$1 ORDER BY created_at"#,
            pid
        )
        .fetch_all(self.db)
        .await?)
    }
}

pub struct DispenseRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> DispenseRepo<'a> {
    pub async fn create(&self, d: &Dispense) -> anyhow::Result<()> {
        sqlx::query!(
            r#"INSERT INTO dispenses(id,prescription_id,disp_no,dispensed_by,status) VALUES($1,$2,$3,$4,$5)"#,
            d.id, d.prescription_id, d.disp_no, d.dispensed_by, d.status
        )
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn finish(&self, id: Uuid) -> anyhow::Result<Option<Dispense>> {
        let rec = sqlx::query_as!(
            Dispense,
            r#"UPDATE dispenses SET status='COMPLETED', updated_at=NOW() WHERE id=$1 RETURNING id,prescription_id,disp_no,dispensed_by,status,created_at,updated_at"#,
            id
        )
        .fetch_optional(self.db)
        .await?;
        Ok(rec)
    }

    pub async fn list_paged(
        &self,
        prescription_id: Option<Uuid>,
        page: i64,
        size: i64,
    ) -> anyhow::Result<(Vec<Dispense>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;
        let (rows, total) = match prescription_id {
            Some(pid) => {
                let r = sqlx::query_as!(
                    Dispense,
                    r#"SELECT id,prescription_id,disp_no,dispensed_by,status,created_at,updated_at FROM dispenses WHERE prescription_id=$1 ORDER BY created_at DESC OFFSET $2 LIMIT $3"#,
                    pid, offset, size
                )
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar!(
                    "SELECT COUNT(1) FROM dispenses WHERE prescription_id=$1",
                    pid
                )
                .fetch_one(self.db)
                .await?;
                (r, t)
            }
            None => {
                let r = sqlx::query_as!(
                    Dispense,
                    r#"SELECT id,prescription_id,disp_no,dispensed_by,status,created_at,updated_at FROM dispenses ORDER BY created_at DESC OFFSET $1 LIMIT $2"#,
                    offset, size
                )
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar!("SELECT COUNT(1) FROM dispenses")
                    .fetch_one(self.db)
                    .await?;
                (r, t)
            }
        };
        Ok((rows, total))
    }
}

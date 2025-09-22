use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::domain::models::*;

pub struct ProcRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> ProcRepo<'a> {
    pub async fn create(&self, p: &Procedure) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO rad_procedures(id,code,name,modality,body_part,contrast,duration_min) VALUES($1,$2,$3,$4,$5,$6,$7)"#
        )
        .bind(&p.id)
        .bind(&p.code)
        .bind(&p.name)
        .bind(&p.modality)
        .bind(&p.body_part)
        .bind(p.contrast)
        .bind(&p.duration_min)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn update(&self, id: Uuid, name: Option<&str>, body_part: Option<&str>, contrast: Option<bool>, duration_min: Option<i32>) -> anyhow::Result<Option<Procedure>> {
        Ok(sqlx::query_as::<_, Procedure>(
            r#"UPDATE rad_procedures SET name=COALESCE($2,name), body_part=COALESCE($3,body_part), contrast=COALESCE($4,contrast), duration_min=COALESCE($5,duration_min), updated_at=NOW() WHERE id=$1 RETURNING id,code,name,modality,body_part,contrast,duration_min,created_at,updated_at"#
        )
        .bind(id)
        .bind(name)
        .bind(body_part)
        .bind(contrast)
        .bind(duration_min)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn search_paged(&self, q: Option<&str>, modality: Option<&str>, page: i64, size: i64) -> anyhow::Result<(Vec<Procedure>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;
        let (rows, total) = match (q, modality) {
            (Some(q), Some(m)) => {
                let like = format!("%{}%", q);
                let r = sqlx::query_as::<_, Procedure>(
                    r#"SELECT id,code,name,modality,body_part,contrast,duration_min,created_at,updated_at FROM rad_procedures WHERE modality=$1 AND (code ILIKE $2 OR name ILIKE $2) ORDER BY code OFFSET $3 LIMIT $4"#
                )
                .bind(m)
                .bind(&like)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>(
                    "SELECT COUNT(1) FROM rad_procedures WHERE modality=$1 AND (code ILIKE $2 OR name ILIKE $2)"
                )
                .bind(m)
                .bind(&like)
                .fetch_one(self.db)
                .await?;
                (r, t)
            },
            (None, Some(m)) => {
                let r = sqlx::query_as::<_, Procedure>(
                    r#"SELECT id,code,name,modality,body_part,contrast,duration_min,created_at,updated_at FROM rad_procedures WHERE modality=$1 ORDER BY code OFFSET $2 LIMIT $3"#
                )
                .bind(m)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>(
                    "SELECT COUNT(1) FROM rad_procedures WHERE modality=$1"
                )
                .bind(m)
                .fetch_one(self.db)
                .await?;
                (r, t)
            },
            (Some(q), None) => {
                let like = format!("%{}%", q);
                let r = sqlx::query_as::<_, Procedure>(
                    r#"SELECT id,code,name,modality,body_part,contrast,duration_min,created_at,updated_at FROM rad_procedures WHERE code ILIKE $1 OR name ILIKE $1 ORDER BY code OFFSET $2 LIMIT $3"#
                )
                .bind(&like)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>(
                    "SELECT COUNT(1) FROM rad_procedures WHERE code ILIKE $1 OR name ILIKE $1"
                )
                .bind(&like)
                .fetch_one(self.db)
                .await?;
                (r, t)
            },
            _ => {
                let r = sqlx::query_as::<_, Procedure>(
                    r#"SELECT id,code,name,modality,body_part,contrast,duration_min,created_at,updated_at FROM rad_procedures ORDER BY code OFFSET $1 LIMIT $2"#
                )
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM rad_procedures")
                    .fetch_one(self.db)
                    .await?;
                (r, t)
            }
        };
        Ok((rows, total))
    }
}

pub struct OrderRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> OrderRepo<'a> {
    pub async fn create(&self, o: &RadOrder) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO rad_orders(id,order_no,patient_id,encounter_id,procedure_id,reason,priority,status,requested_by,scheduled_at) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)"#
        )
        .bind(&o.id)
        .bind(&o.order_no)
        .bind(&o.patient_id)
        .bind(&o.encounter_id)
        .bind(&o.procedure_id)
        .bind(&o.reason)
        .bind(&o.priority)
        .bind(&o.status)
        .bind(&o.requested_by)
        .bind(&o.scheduled_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn update(&self, id: Uuid, status: Option<&str>, scheduled_at: Option<chrono::DateTime<chrono::Utc>>) -> anyhow::Result<Option<RadOrder>> {
        Ok(sqlx::query_as::<_, RadOrder>(
            r#"UPDATE rad_orders SET status=COALESCE($2,status), scheduled_at=COALESCE($3,scheduled_at), updated_at=NOW() WHERE id=$1 RETURNING id,order_no,patient_id,encounter_id,procedure_id,reason,priority,status,requested_by,scheduled_at,created_at,updated_at"#
        )
        .bind(id)
        .bind(status)
        .bind(scheduled_at)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list_paged(&self, patient_id: Option<Uuid>, status: Option<&str>, page: i64, size: i64) -> anyhow::Result<(Vec<RadOrder>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;
        let (rows, total) = match (patient_id, status) {
            (Some(p), Some(s)) => {
                let r = sqlx::query_as::<_, RadOrder>(
                    r#"SELECT id,order_no,patient_id,encounter_id,procedure_id,reason,priority,status,requested_by,scheduled_at,created_at,updated_at FROM rad_orders WHERE patient_id=$1 AND status=$2 ORDER BY created_at DESC OFFSET $3 LIMIT $4"#
                )
                .bind(p)
                .bind(s)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>(
                    "SELECT COUNT(1) FROM rad_orders WHERE patient_id=$1 AND status=$2"
                )
                .bind(p)
                .bind(s)
                .fetch_one(self.db)
                .await?;
                (r, t)
            },
            (Some(p), None) => {
                let r = sqlx::query_as::<_, RadOrder>(
                    r#"SELECT id,order_no,patient_id,encounter_id,procedure_id,reason,priority,status,requested_by,scheduled_at,created_at,updated_at FROM rad_orders WHERE patient_id=$1 ORDER BY created_at DESC OFFSET $2 LIMIT $3"#
                )
                .bind(p)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>(
                    "SELECT COUNT(1) FROM rad_orders WHERE patient_id=$1"
                )
                .bind(p)
                .fetch_one(self.db)
                .await?;
                (r, t)
            },
            _ => {
                let r = sqlx::query_as::<_, RadOrder>(
                    r#"SELECT id,order_no,patient_id,encounter_id,procedure_id,reason,priority,status,requested_by,scheduled_at,created_at,updated_at FROM rad_orders ORDER BY created_at DESC OFFSET $1 LIMIT $2"#
                )
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM rad_orders")
                    .fetch_one(self.db)
                    .await?;
                (r, t)
            }
        };
        Ok((rows, total))
    }

    pub async fn find(&self, id: Uuid) -> anyhow::Result<Option<RadOrder>> {
        Ok(sqlx::query_as::<_, RadOrder>(
            r#"SELECT id,order_no,patient_id,encounter_id,procedure_id,reason,priority,status,requested_by,scheduled_at,created_at,updated_at FROM rad_orders WHERE id=$1"#
        )
        .bind(id)
        .fetch_optional(self.db)
        .await?)
    }
}

pub struct StudyRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> StudyRepo<'a> {
    pub async fn create(&self, s: &Study) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO rad_studies(id,study_uid,order_id,accession_no,modality,started_at,ended_at,performer,status) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9)"#
        )
        .bind(&s.id)
        .bind(&s.study_uid)
        .bind(&s.order_id)
        .bind(&s.accession_no)
        .bind(&s.modality)
        .bind(&s.started_at)
        .bind(&s.ended_at)
        .bind(&s.performer)
        .bind(&s.status)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn set_progress(&self, id: Uuid, action: &str, performer: Option<&str>) -> anyhow::Result<Option<Study>> {
        let rec = match action {
            "START" => sqlx::query_as::<_, Study>(
                r#"UPDATE rad_studies SET status='INPROGRESS', started_at=NOW(), performer=COALESCE($2,performer), updated_at=NOW() WHERE id=$1 RETURNING id,study_uid,order_id,accession_no,modality,started_at,ended_at,performer,status,created_at,updated_at"#
            )
            .bind(id)
            .bind(performer)
            .fetch_optional(self.db)
            .await?,
            "END" => sqlx::query_as::<_, Study>(
                r#"UPDATE rad_studies SET status='DONE', ended_at=NOW(), updated_at=NOW() WHERE id=$1 RETURNING id,study_uid,order_id,accession_no,modality,started_at,ended_at,performer,status,created_at,updated_at"#
            )
            .bind(id)
            .fetch_optional(self.db)
            .await?,
            _ => None,
        };
        Ok(rec)
    }

    pub async fn list_paged(&self, order_id: Option<Uuid>, status: Option<&str>, page: i64, size: i64) -> anyhow::Result<(Vec<Study>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;
        let (rows, total) = match (order_id, status) {
            (Some(o), Some(s)) => {
                let r = sqlx::query_as::<_, Study>(
                    r#"SELECT id,study_uid,order_id,accession_no,modality,started_at,ended_at,performer,status,created_at,updated_at FROM rad_studies WHERE order_id=$1 AND status=$2 ORDER BY created_at DESC OFFSET $3 LIMIT $4"#
                )
                .bind(o)
                .bind(s)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>(
                    "SELECT COUNT(1) FROM rad_studies WHERE order_id=$1 AND status=$2"
                )
                .bind(o)
                .bind(s)
                .fetch_one(self.db)
                .await?;
                (r, t)
            },
            _ => {
                let r = sqlx::query_as::<_, Study>(
                    r#"SELECT id,study_uid,order_id,accession_no,modality,started_at,ended_at,performer,status,created_at,updated_at FROM rad_studies ORDER BY created_at DESC OFFSET $1 LIMIT $2"#
                )
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM rad_studies")
                    .fetch_one(self.db)
                    .await?;
                (r, t)
            }
        };
        Ok((rows, total))
    }

    pub async fn find(&self, id: Uuid) -> anyhow::Result<Option<Study>> {
        Ok(sqlx::query_as::<_, Study>(
            r#"SELECT id,study_uid,order_id,accession_no,modality,started_at,ended_at,performer,status,created_at,updated_at FROM rad_studies WHERE id=$1"#
        )
        .bind(id)
        .fetch_optional(self.db)
        .await?)
    }
}

pub struct ReportRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> ReportRepo<'a> {
    pub async fn create(&self, r: &Report) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO rad_reports(id,report_no,study_id,status,content,author,verified_by,verified_at,finalized_by,finalized_at) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)"#
        )
        .bind(&r.id)
        .bind(&r.report_no)
        .bind(&r.study_id)
        .bind(&r.status)
        .bind(&r.content)
        .bind(&r.author)
        .bind(&r.verified_by)
        .bind(&r.verified_at)
        .bind(&r.finalized_by)
        .bind(&r.finalized_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn list_paged(&self, study_id: Option<Uuid>, status: Option<&str>, page: i64, size: i64) -> anyhow::Result<(Vec<Report>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;
        let (rows, total) = match (study_id, status) {
            (Some(sid), Some(st)) => {
                let r = sqlx::query_as::<_, Report>(
                    r#"SELECT id,report_no,study_id,status,content,author,verified_by,verified_at,finalized_by,finalized_at,created_at,updated_at FROM rad_reports WHERE study_id=$1 AND status=$2 ORDER BY created_at DESC OFFSET $3 LIMIT $4"#
                )
                .bind(sid)
                .bind(st)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>(
                    "SELECT COUNT(1) FROM rad_reports WHERE study_id=$1 AND status=$2"
                )
                .bind(sid)
                .bind(st)
                .fetch_one(self.db)
                .await?;
                (r, t)
            },
            _ => {
                let r = sqlx::query_as::<_, Report>(
                    r#"SELECT id,report_no,study_id,status,content,author,verified_by,verified_at,finalized_by,finalized_at,created_at,updated_at FROM rad_reports ORDER BY created_at DESC OFFSET $1 LIMIT $2"#
                )
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t = sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM rad_reports")
                    .fetch_one(self.db)
                    .await?;
                (r, t)
            }
        };
        Ok((rows, total))
    }

    pub async fn set_content(&self, id: Uuid, content: &str, author: &str) -> anyhow::Result<Option<Report>> {
        Ok(sqlx::query_as::<_, Report>(
            r#"UPDATE rad_reports SET content=$2, author=COALESCE(author,$3), updated_at=NOW() WHERE id=$1 RETURNING id,report_no,study_id,status,content,author,verified_by,verified_at,finalized_by,finalized_at,created_at,updated_at"#
        )
        .bind(id)
        .bind(content)
        .bind(author)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn set_status(&self, id: Uuid, status: &str, by: Option<&str>) -> anyhow::Result<Option<Report>> {
        let (ver_by, fin_by) = match status {
            "PRELIM" => (by, None),
            "FINAL" => (None, by),
            _ => (None, None),
        };
        let rec = sqlx::query_as::<_, Report>(
            r#"UPDATE rad_reports SET status=$2, verified_by=COALESCE($3,verified_by), verified_at=CASE WHEN $2='PRELIM' THEN NOW() ELSE verified_at END, finalized_by=COALESCE($4,finalized_by), finalized_at=CASE WHEN $2='FINAL' THEN NOW() ELSE finalized_at END, updated_at=NOW() WHERE id=$1 RETURNING id,report_no,study_id,status,content,author,verified_by,verified_at,finalized_by,finalized_at,created_at,updated_at"#
        )
        .bind(id)
        .bind(status)
        .bind(ver_by)
        .bind(fin_by)
        .fetch_optional(self.db)
        .await?;
        Ok(rec)
    }

    pub async fn find(&self, id: Uuid) -> anyhow::Result<Option<Report>> {
        Ok(sqlx::query_as::<_, Report>(
            r#"SELECT id,report_no,study_id,status,content,author,verified_by,verified_at,finalized_by,finalized_at,created_at,updated_at FROM rad_reports WHERE id=$1"#
        )
        .bind(id)
        .fetch_optional(self.db)
        .await?)
    }
}

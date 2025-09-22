use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::domain::models::*;

pub struct ProviderRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> ProviderRepo<'a> {
    pub async fn create(&self, p: &Provider) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO providers(id,code,name,specialty) VALUES($1,$2,$3,$4)"
        )
        .bind(p.id)
        .bind(&p.code)
        .bind(&p.name)
        .bind(&p.specialty)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn update(&self, id: Uuid, name: Option<&str>, spec: Option<&str>) -> anyhow::Result<Option<Provider>> {
        Ok(sqlx::query_as::<_, Provider>(
            r#"UPDATE providers SET name=COALESCE($2,name), specialty=COALESCE($3,specialty), updated_at=NOW() WHERE id=$1 RETURNING id,code,name,specialty,created_at,updated_at"#
        )
        .bind(id)
        .bind(name)
        .bind(spec)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn search_paged(&self, q: Option<&str>, page: i64, size: i64) -> anyhow::Result<(Vec<Provider>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;
        let (rows, total) = if let Some(q) = q {
            let like = format!("%{}%", q);
            let r = sqlx::query_as::<_, Provider>(
                r#"SELECT id,code,name,specialty,created_at,updated_at FROM providers WHERE code ILIKE $1 OR name ILIKE $1 ORDER BY code OFFSET $2 LIMIT $3"#
            )
            .bind(&like)
            .bind(offset)
            .bind(size)
            .fetch_all(self.db)
            .await?;
            let t: i64 = sqlx::query_scalar(
                "SELECT COUNT(1) FROM providers WHERE code ILIKE $1 OR name ILIKE $1"
            )
            .bind(&like)
            .fetch_one(self.db)
            .await?;
            (r, t)
        } else {
            let r = sqlx::query_as::<_, Provider>(
                r#"SELECT id,code,name,specialty,created_at,updated_at FROM providers ORDER BY code OFFSET $1 LIMIT $2"#
            )
            .bind(offset)
            .bind(size)
            .fetch_all(self.db)
            .await?;
            let t: i64 = sqlx::query_scalar("SELECT COUNT(1) FROM providers")
                .fetch_one(self.db)
                .await?;
            (r, t)
        };
        Ok((rows, total))
    }
}

pub struct RoomRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> RoomRepo<'a> {
    pub async fn create(&self, r: &Room) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO rooms(id,code,name,location) VALUES($1,$2,$3,$4)"
        )
        .bind(r.id)
        .bind(&r.code)
        .bind(&r.name)
        .bind(&r.location)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn update(&self, id: Uuid, name: Option<&str>, loc: Option<&str>) -> anyhow::Result<Option<Room>> {
        Ok(sqlx::query_as::<_, Room>(
            r#"UPDATE rooms SET name=COALESCE($2,name), location=COALESCE($3,location), updated_at=NOW() WHERE id=$1 RETURNING id,code,name,location,created_at,updated_at"#
        )
        .bind(id)
        .bind(name)
        .bind(loc)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn search_paged(&self, q: Option<&str>, page: i64, size: i64) -> anyhow::Result<(Vec<Room>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;
        let (rows, total) = if let Some(q) = q {
            let like = format!("%{}%", q);
            let r = sqlx::query_as::<_, Room>(
                r#"SELECT id,code,name,location,created_at,updated_at FROM rooms WHERE code ILIKE $1 OR name ILIKE $1 ORDER BY code OFFSET $2 LIMIT $3"#
            )
            .bind(&like)
            .bind(offset)
            .bind(size)
            .fetch_all(self.db)
            .await?;
            let t: i64 = sqlx::query_scalar(
                "SELECT COUNT(1) FROM rooms WHERE code ILIKE $1 OR name ILIKE $1"
            )
            .bind(&like)
            .fetch_one(self.db)
            .await?;
            (r, t)
        } else {
            let r = sqlx::query_as::<_, Room>(
                r#"SELECT id,code,name,location,created_at,updated_at FROM rooms ORDER BY code OFFSET $1 LIMIT $2"#
            )
            .bind(offset)
            .bind(size)
            .fetch_all(self.db)
            .await?;
            let t: i64 = sqlx::query_scalar("SELECT COUNT(1) FROM rooms")
                .fetch_one(self.db)
                .await?;
            (r, t)
        };
        Ok((rows, total))
    }
}

pub struct ScheduleRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> ScheduleRepo<'a> {
    pub async fn create(&self, s: &Schedule) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO schedules(id,provider_id,room_id,weekday,start_time,end_time,slot_min,active) VALUES($1,$2,$3,$4,$5,$6,$7,$8)"
        )
        .bind(s.id)
        .bind(s.provider_id)
        .bind(s.room_id)
        .bind(s.weekday)
        .bind(s.start_time)
        .bind(s.end_time)
        .bind(s.slot_min)
        .bind(s.active)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn update(&self, id: Uuid, room_id: Option<Uuid>, weekday: Option<i16>, st: Option<chrono::NaiveTime>, et: Option<chrono::NaiveTime>, slot_min: Option<i16>, active: Option<bool>) -> anyhow::Result<Option<Schedule>> {
        Ok(sqlx::query_as::<_, Schedule>(
            r#"UPDATE schedules SET room_id=COALESCE($2,room_id), weekday=COALESCE($3,weekday), start_time=COALESCE($4,start_time), end_time=COALESCE($5,end_time), slot_min=COALESCE($6,slot_min), active=COALESCE($7,active), updated_at=NOW() WHERE id=$1 RETURNING id,provider_id,room_id,weekday,start_time,end_time,slot_min,active,created_at,updated_at"#
        )
        .bind(id)
        .bind(room_id)
        .bind(weekday)
        .bind(st)
        .bind(et)
        .bind(slot_min)
        .bind(active)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list_paged(&self, provider_id: Option<Uuid>, weekday: Option<i16>, page: i64, size: i64) -> anyhow::Result<(Vec<Schedule>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;
        let (rows, total) = match (provider_id, weekday) {
            (Some(p), Some(w)) => {
                let r = sqlx::query_as::<_, Schedule>(
                    r#"SELECT id,provider_id,room_id,weekday,start_time,end_time,slot_min,active,created_at,updated_at FROM schedules WHERE provider_id=$1 AND weekday=$2 ORDER BY start_time OFFSET $3 LIMIT $4"#
                )
                .bind(p)
                .bind(w)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t: i64 = sqlx::query_scalar(
                    "SELECT COUNT(1) FROM schedules WHERE provider_id=$1 AND weekday=$2"
                )
                .bind(p)
                .bind(w)
                .fetch_one(self.db)
                .await?;
                (r, t)
            },
            (Some(p), None) => {
                let r = sqlx::query_as::<_, Schedule>(
                    r#"SELECT id,provider_id,room_id,weekday,start_time,end_time,slot_min,active,created_at,updated_at FROM schedules WHERE provider_id=$1 ORDER BY weekday,start_time OFFSET $2 LIMIT $3"#
                )
                .bind(p)
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t: i64 = sqlx::query_scalar(
                    "SELECT COUNT(1) FROM schedules WHERE provider_id=$1"
                )
                .bind(p)
                .fetch_one(self.db)
                .await?;
                (r, t)
            },
            _ => {
                let r = sqlx::query_as::<_, Schedule>(
                    r#"SELECT id,provider_id,room_id,weekday,start_time,end_time,slot_min,active,created_at,updated_at FROM schedules ORDER BY provider_id,weekday,start_time OFFSET $1 LIMIT $2"#
                )
                .bind(offset)
                .bind(size)
                .fetch_all(self.db)
                .await?;
                let t: i64 = sqlx::query_scalar("SELECT COUNT(1) FROM schedules")
                    .fetch_one(self.db)
                    .await?;
                (r, t)
            }
        };
        Ok((rows, total))
    }
}

pub struct SlotRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> SlotRepo<'a> {
    pub async fn upsert(&self, s: &TimeSlot) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO time_slots(id,provider_id,room_id,starts_at,ends_at,reserved,locked_by) VALUES($1,$2,$3,$4,$5,$6,$7) ON CONFLICT (provider_id,starts_at,ends_at) DO NOTHING"#
        )
        .bind(s.id)
        .bind(s.provider_id)
        .bind(s.room_id)
        .bind(s.starts_at)
        .bind(s.ends_at)
        .bind(s.reserved)
        .bind(&s.locked_by)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn list_range(&self, provider_id: Uuid, from: chrono::DateTime<chrono::Utc>, to: chrono::DateTime<chrono::Utc>, only_free: bool, page: i64, size: i64) -> anyhow::Result<(Vec<TimeSlot>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;
        let (rows, total) = if only_free {
            let r = sqlx::query_as::<_, TimeSlot>(
                r#"SELECT id,provider_id,room_id,starts_at,ends_at,reserved,locked_by FROM time_slots WHERE provider_id=$1 AND starts_at >= $2 AND ends_at <= $3 AND reserved = FALSE ORDER BY starts_at OFFSET $4 LIMIT $5"#
            )
            .bind(provider_id)
            .bind(from)
            .bind(to)
            .bind(offset)
            .bind(size)
            .fetch_all(self.db)
            .await?;
            let t: i64 = sqlx::query_scalar(
                "SELECT COUNT(1) FROM time_slots WHERE provider_id=$1 AND starts_at >= $2 AND ends_at <= $3 AND reserved = FALSE"
            )
            .bind(provider_id)
            .bind(from)
            .bind(to)
            .fetch_one(self.db)
            .await?;
            (r, t)
        } else {
            let r = sqlx::query_as::<_, TimeSlot>(
                r#"SELECT id,provider_id,room_id,starts_at,ends_at,reserved,locked_by FROM time_slots WHERE provider_id=$1 AND starts_at >= $2 AND ends_at <= $3 ORDER BY starts_at OFFSET $4 LIMIT $5"#
            )
            .bind(provider_id)
            .bind(from)
            .bind(to)
            .bind(offset)
            .bind(size)
            .fetch_all(self.db)
            .await?;
            let t: i64 = sqlx::query_scalar(
                "SELECT COUNT(1) FROM time_slots WHERE provider_id=$1 AND starts_at >= $2 AND ends_at <= $3"
            )
            .bind(provider_id)
            .bind(from)
            .bind(to)
            .fetch_one(self.db)
            .await?;
            (r, t)
        };
        Ok((rows, total))
    }

    pub async fn lock_free_slot(&self, slot_id: Uuid, locker: &str, tx: &mut sqlx::Transaction<'_, Postgres>) -> anyhow::Result<bool> {
        let rec = sqlx::query(
            "UPDATE time_slots SET reserved=TRUE, locked_by=$2 WHERE id=$1 AND reserved=FALSE"
        )
        .bind(slot_id)
        .bind(locker)
        .execute(tx.as_mut())
        .await?;
        Ok(rec.rows_affected() == 1)
    }
}

pub struct ApptRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> ApptRepo<'a> {
    pub async fn create(&self, a: &Appointment, tx: &mut sqlx::Transaction<'_, Postgres>) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO appointments(id,appt_no,patient_id,provider_id,room_id,slot_id,status,reason,created_by) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9)"
        )
        .bind(a.id)
        .bind(&a.appt_no)
        .bind(a.patient_id)
        .bind(a.provider_id)
        .bind(a.room_id)
        .bind(a.slot_id)
        .bind(&a.status)
        .bind(&a.reason)
        .bind(&a.created_by)
        .execute(tx.as_mut())
        .await?;
        Ok(())
    }

    pub async fn set_status(&self, id: Uuid, status: &str) -> anyhow::Result<Option<Appointment>> {
        Ok(sqlx::query_as::<_, Appointment>(
            r#"UPDATE appointments SET status=$2, updated_at=NOW() WHERE id=$1 RETURNING id,appt_no,patient_id,provider_id,room_id,slot_id,status,reason,created_by,created_at,updated_at"#
        )
        .bind(id)
        .bind(status)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list_paged(&self, patient_id: Option<Uuid>, provider_id: Option<Uuid>, status: Option<&str>, page: i64, size: i64) -> anyhow::Result<(Vec<Appointment>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;
        let where_clause = match (patient_id, provider_id, status) {
            (Some(p), Some(d), Some(s)) => format!("WHERE patient_id='{}' AND provider_id='{}' AND status='{}'", p, d, s),
            (Some(p), Some(d), None) => format!("WHERE patient_id='{}' AND provider_id='{}'", p, d),
            (Some(p), None, Some(s)) => format!("WHERE patient_id='{}' AND status='{}'", p, s),
            (None, Some(d), Some(s)) => format!("WHERE provider_id='{}' AND status='{}'", d, s),
            (Some(p), None, None) => format!("WHERE patient_id='{}'", p),
            (None, Some(d), None) => format!("WHERE provider_id='{}'", d),
            (None, None, Some(s)) => format!("WHERE status='{}'", s),
            _ => String::new(),
        };
        let q = format!(
            "SELECT id,appt_no,patient_id,provider_id,room_id,slot_id,status,reason,created_by,created_at,updated_at FROM appointments {} ORDER BY created_at DESC OFFSET $1 LIMIT $2",
            where_clause
        );
        let rows: Vec<Appointment> = sqlx::query_as(&q)
            .bind(offset)
            .bind(size)
            .fetch_all(self.db)
            .await?;
        let q2 = format!("SELECT COUNT(1) FROM appointments {}", where_clause);
        let total: i64 = sqlx::query_scalar(&q2)
            .fetch_one(self.db)
            .await?;
        Ok((rows, total))
    }
}

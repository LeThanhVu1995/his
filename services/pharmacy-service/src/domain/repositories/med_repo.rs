use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::domain::models::Medication;

pub struct MedRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> MedRepo<'a> {
    pub async fn create(&self, m: &Medication) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO medications(id,code,name,strength,form,route) VALUES($1,$2,$3,$4,$5,$6)"#
        )
        .bind(m.id)
        .bind(&m.code)
        .bind(&m.name)
        .bind(&m.strength)
        .bind(&m.form)
        .bind(&m.route)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn update(&self, id: Uuid, name: Option<&str>, strength: Option<&str>, form: Option<&str>, route: Option<&str>) -> anyhow::Result<Option<Medication>> {
        let rec = sqlx::query_as::<_, Medication>(
            r#"UPDATE medications SET name=COALESCE($2,name), strength=COALESCE($3,strength), form=COALESCE($4,form), route=COALESCE($5,route), updated_at=NOW() WHERE id=$1 RETURNING id,code,name,strength,form,route,created_at,updated_at"#
        )
        .bind(id)
        .bind(name)
        .bind(strength)
        .bind(form)
        .bind(route)
        .fetch_optional(self.db)
        .await?;
        Ok(rec)
    }

    pub async fn search_paged(&self, q: Option<&str>, page: i64, size: i64) -> anyhow::Result<(Vec<Medication>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;

        let (rows, total) = if let Some(q) = q {
            let like = format!("%{}%", q);
            let r = sqlx::query_as::<_, Medication>(
                r#"SELECT id,code,name,strength,form,route,created_at,updated_at FROM medications WHERE code ILIKE $1 OR name ILIKE $1 ORDER BY name OFFSET $2 LIMIT $3"#
            )
            .bind(&like)
            .bind(offset)
            .bind(size)
            .fetch_all(self.db)
            .await?;
            let t = sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM medications WHERE code ILIKE $1 OR name ILIKE $1")
                .bind(&like)
                .fetch_one(self.db)
                .await?;
            (r, t)
        } else {
            let r = sqlx::query_as::<_, Medication>(
                r#"SELECT id,code,name,strength,form,route,created_at,updated_at FROM medications ORDER BY name OFFSET $1 LIMIT $2"#
            )
            .bind(offset)
            .bind(size)
            .fetch_all(self.db)
            .await?;
            let t = sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM medications")
                .fetch_one(self.db)
                .await?;
            (r, t)
        };
        Ok((rows, total))
    }

    pub async fn find(&self, id: Uuid) -> anyhow::Result<Option<Medication>> {
        Ok(sqlx::query_as::<_, Medication>(
            r#"SELECT id,code,name,strength,form,route,created_at,updated_at FROM medications WHERE id=$1"#
        )
        .bind(id)
        .fetch_optional(self.db)
        .await?)
    }
}

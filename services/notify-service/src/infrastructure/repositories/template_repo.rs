use uuid::Uuid;
use sqlx::{Pool, Postgres};
use crate::domain::entities::template::Template;

pub struct TemplateRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> TemplateRepo<'a> {
    pub async fn create(&self, t: &Template) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO notify_templates(id,code,name,channel,subject,body,is_active) VALUES($1,$2,$3,$4,$5,$6,$7)"#
        )
        .bind(t.id)
        .bind(&t.code)
        .bind(&t.name)
        .bind(&t.channel)
        .bind(&t.subject)
        .bind(&t.body)
        .bind(t.is_active)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn update(&self, id: Uuid, name: Option<&str>, subject: Option<&str>, body: Option<&str>, active: Option<bool>) -> anyhow::Result<Option<Template>> {
        Ok(sqlx::query_as::<_, Template>(
            r#"UPDATE notify_templates SET name=COALESCE($2,name), subject=COALESCE($3,subject), body=COALESCE($4,body), is_active=COALESCE($5,is_active), updated_at=NOW() WHERE id=$1 RETURNING id,code,name,channel,subject,body,is_active,created_at,updated_at"#
        )
        .bind(id)
        .bind(name)
        .bind(subject)
        .bind(body)
        .bind(active)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn get_by_code(&self, code: &str) -> anyhow::Result<Option<Template>> {
        Ok(sqlx::query_as::<_, Template>(
            r#"SELECT id,code,name,channel,subject,body,is_active,created_at,updated_at FROM notify_templates WHERE code=$1"#
        )
        .bind(code)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list(&self, channel: Option<&str>, page: i64, size: i64) -> anyhow::Result<(Vec<Template>, i64)> {
        let page = page.max(1);
        let size = size.clamp(1, 200);
        let offset = (page - 1) * size;

        let (rows, total) = if let Some(ch) = channel {
            let r = sqlx::query_as::<_, Template>(
                r#"SELECT id,code,name,channel,subject,body,is_active,created_at,updated_at FROM notify_templates WHERE channel=$1 ORDER BY created_at DESC OFFSET $2 LIMIT $3"#
            )
            .bind(ch)
            .bind(offset)
            .bind(size)
            .fetch_all(self.db)
            .await?;
            let t = sqlx::query_scalar::<_, i64>(
                r#"SELECT COUNT(1) FROM notify_templates WHERE channel=$1"#
            )
            .bind(ch)
            .fetch_one(self.db)
            .await?;
            (r, t)
        } else {
            let r = sqlx::query_as::<_, Template>(
                r#"SELECT id,code,name,channel,subject,body,is_active,created_at,updated_at FROM notify_templates ORDER BY created_at DESC OFFSET $1 LIMIT $2"#
            )
            .bind(offset)
            .bind(size)
            .fetch_all(self.db)
            .await?;
            let t = sqlx::query_scalar::<_, i64>(
                r#"SELECT COUNT(1) FROM notify_templates"#
            )
            .fetch_one(self.db)
            .await?;
            (r, t)
        };
        Ok((rows, total))
    }
}

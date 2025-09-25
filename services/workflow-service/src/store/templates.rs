use uuid::Uuid;
use sqlx::{Pool, Postgres};

pub struct TemplateStore<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> TemplateStore<'a> {
    pub async fn upsert(
        &self,
        code: &str,
        name: &str,
        version: i32,
        spec: &serde_json::Value,
    ) -> anyhow::Result<Uuid> {
        let id = Uuid::new_v4();
        sqlx::query(
            r#"INSERT INTO wf_templates(id,code,name,version,spec,is_active)
               VALUES($1,$2,$3,$4,$5,TRUE)
               ON CONFLICT(code) DO UPDATE SET
                   name=EXCLUDED.name,
                   version=EXCLUDED.version,
                   spec=EXCLUDED.spec,
                   updated_at=NOW()"#
        )
        .bind(id)
        .bind(code)
        .bind(name)
        .bind(version)
        .bind(spec)
        .execute(self.db)
        .await?;
        Ok(id)
    }

    pub async fn get(&self, code: &str) -> anyhow::Result<Option<serde_json::Value>> {
        Ok(sqlx::query_scalar("SELECT spec FROM wf_templates WHERE code=$1 AND is_active=TRUE")
            .bind(code)
            .fetch_optional(self.db)
            .await?)
    }

    pub async fn list(&self) -> anyhow::Result<Vec<crate::domain::entities::template::Template>> {
        Ok(sqlx::query_as::<_, crate::domain::entities::template::Template>(
            r#"SELECT id,code,name,version,spec,is_active,created_at,updated_at
               FROM wf_templates
               WHERE is_active=TRUE
               ORDER BY created_at DESC"#
        )
        .fetch_all(self.db)
        .await?)
    }
}

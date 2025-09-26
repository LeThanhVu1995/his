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

        // Check if template exists and get current version
        let current_version = sqlx::query_scalar::<_, i32>(
            "SELECT version FROM wf_templates WHERE code=$1 ORDER BY version DESC LIMIT 1"
        )
        .bind(code)
        .fetch_optional(self.db)
        .await?;

        let new_version = if let Some(current) = current_version {
            if version <= current {
                current + 1  // Auto-increment if version not specified or lower
            } else {
                version
            }
        } else {
            version
        };

        sqlx::query(
            r#"INSERT INTO wf_templates(id,code,name,version,spec,is_active)
               VALUES($1,$2,$3,$4,$5,TRUE)"#
        )
        .bind(id)
        .bind(code)
        .bind(name)
        .bind(new_version)
        .bind(spec)
        .execute(self.db)
        .await?;
        Ok(id)
    }

    pub async fn get(&self, code: &str) -> anyhow::Result<Option<serde_json::Value>> {
        Ok(sqlx::query_scalar("SELECT spec FROM wf_templates WHERE code=$1 AND is_active=TRUE ORDER BY version DESC LIMIT 1")
            .bind(code)
            .fetch_optional(self.db)
            .await?)
    }

    pub async fn get_by_version(&self, code: &str, version: i32) -> anyhow::Result<Option<serde_json::Value>> {
        Ok(sqlx::query_scalar("SELECT spec FROM wf_templates WHERE code=$1 AND version=$2 AND is_active=TRUE")
            .bind(code)
            .bind(version)
            .fetch_optional(self.db)
            .await?)
    }

    pub async fn get_latest_version(&self, code: &str) -> anyhow::Result<Option<i32>> {
        Ok(sqlx::query_scalar("SELECT version FROM wf_templates WHERE code=$1 AND is_active=TRUE ORDER BY version DESC LIMIT 1")
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

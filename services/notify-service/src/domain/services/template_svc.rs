use crate::infrastructure::repositories::template_repo::TemplateRepo;
use crate::domain::entities::template::Template;
use uuid::Uuid;

pub struct TemplateSvc<'a> {
    pub db: &'a sqlx::Pool<sqlx::Postgres>,
}

impl<'a> TemplateSvc<'a> {
    pub async fn create(&self, t: &Template) -> anyhow::Result<()> {
        let repo = TemplateRepo { db: self.db };
        repo.create(t).await
    }

    pub async fn update(
        &self,
        id: Uuid,
        name: Option<&str>,
        subject: Option<&str>,
        body: Option<&str>,
        active: Option<bool>,
    ) -> anyhow::Result<Option<Template>> {
        let repo = TemplateRepo { db: self.db };
        repo.update(id, name, subject, body, active).await
    }

    pub async fn get_by_code(&self, code: &str) -> anyhow::Result<Option<Template>> {
        let repo = TemplateRepo { db: self.db };
        repo.get_by_code(code).await
    }

    pub async fn list(
        &self,
        channel: Option<&str>,
        page: i64,
        size: i64,
    ) -> anyhow::Result<(Vec<Template>, i64)> {
        let repo = TemplateRepo { db: self.db };
        repo.list(channel, page, size).await
    }
}

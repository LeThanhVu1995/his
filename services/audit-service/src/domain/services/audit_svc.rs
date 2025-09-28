use crate::domain::entities::audit_event::AuditEvent;
use crate::infra::db::repositories::audit_repo::AuditRepo;
use sqlx::Pool;
use sqlx::Postgres;

pub struct AuditSvc<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> AuditSvc<'a> {
    pub async fn create_event(&self, event: &AuditEvent) -> anyhow::Result<()> {
        let repo = AuditRepo { db: self.db };
        repo.insert(event).await
    }

    pub async fn list_events(&self, limit: i64, offset: i64) -> anyhow::Result<Vec<AuditEvent>> {
        let repo = AuditRepo { db: self.db };
        repo.list(limit, offset).await
    }

    pub async fn list_by_user(&self, user_id: &str, limit: i64, offset: i64) -> anyhow::Result<Vec<AuditEvent>> {
        let repo = AuditRepo { db: self.db };
        repo.by_user(user_id, limit, offset).await
    }

    pub async fn list_by_entity(&self, entity_name: &str, entity_id: &str, limit: i64, offset: i64) -> anyhow::Result<Vec<AuditEvent>> {
        let repo = AuditRepo { db: self.db };
        repo.by_entity(entity_name, entity_id, limit, offset).await
    }

    pub async fn list_by_action(&self, action: &str, limit: i64, offset: i64) -> anyhow::Result<Vec<AuditEvent>> {
        let repo = AuditRepo { db: self.db };
        repo.by_action(action, limit, offset).await
    }
}

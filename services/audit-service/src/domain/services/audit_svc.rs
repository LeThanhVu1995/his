use crate::domain::entities::audit_event::AuditEvent;
use crate::infra::db::repositories::audit_repo::AuditRepo;
use sqlx::Pool;
use sqlx::Postgres;
use uuid::Uuid;

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

    pub async fn list_by_actor(&self, actor_id: Uuid, limit: i64, offset: i64) -> anyhow::Result<Vec<AuditEvent>> {
        let repo = AuditRepo { db: self.db };
        repo.by_actor(actor_id, limit, offset).await
    }

    pub async fn list_by_entity(&self, entity_type: &str, entity_id: Uuid, limit: i64, offset: i64) -> anyhow::Result<Vec<AuditEvent>> {
        let repo = AuditRepo { db: self.db };
        repo.by_entity(entity_type, entity_id, limit, offset).await
    }
}

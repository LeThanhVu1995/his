use uuid::Uuid;
use sqlx::{Pool, Postgres};
use crate::domain::entities::audit_event::AuditEvent;

pub struct AuditRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> AuditRepo<'a> {
    pub async fn insert(&self, e: &AuditEvent) -> anyhow::Result<()> {
        sqlx::query(r#"INSERT INTO audit_events(id,occurred_at,actor_id,actor_name,actor_role,ip,user_agent,action,entity_type,entity_id,tenant_id,request_id,source,data,hash) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15)"#)
            .bind(e.id)
            .bind(e.occurred_at)
            .bind(e.actor_id)
            .bind(e.actor_name.clone())
            .bind(e.actor_role.clone())
            .bind(e.ip.clone())
            .bind(e.user_agent.clone())
            .bind(e.action.clone())
            .bind(e.entity_type.clone())
            .bind(e.entity_id)
            .bind(e.tenant_id)
            .bind(e.request_id)
            .bind(e.source.clone())
            .bind(e.data.clone())
            .bind(e.hash.clone())
            .execute(self.db)
            .await?;
        Ok(())
    }

    pub async fn list(&self, limit: i64, offset: i64) -> anyhow::Result<Vec<AuditEvent>> {
        Ok(sqlx::query_as::<_, AuditEvent>(r#"SELECT id,occurred_at,actor_id,actor_name,actor_role,ip::text as ip,user_agent,action,entity_type,entity_id,tenant_id,request_id,source,data,hash,created_at FROM audit_events ORDER BY occurred_at DESC OFFSET $1 LIMIT $2"#)
            .bind(offset)
            .bind(limit)
            .fetch_all(self.db)
            .await?)
    }

    pub async fn by_actor(&self, actor: Uuid, limit: i64, offset: i64) -> anyhow::Result<Vec<AuditEvent>> {
        Ok(sqlx::query_as::<_, AuditEvent>(r#"SELECT id,occurred_at,actor_id,actor_name,actor_role,ip::text as ip,user_agent,action,entity_type,entity_id,tenant_id,request_id,source,data,hash,created_at FROM audit_events WHERE actor_id=$1 ORDER BY occurred_at DESC OFFSET $2 LIMIT $3"#)
            .bind(actor)
            .bind(offset)
            .bind(limit)
            .fetch_all(self.db)
            .await?)
    }

    pub async fn by_entity(&self, et: &str, eid: Uuid, limit: i64, offset: i64) -> anyhow::Result<Vec<AuditEvent>> {
        Ok(sqlx::query_as::<_, AuditEvent>(r#"SELECT id,occurred_at,actor_id,actor_name,actor_role,ip::text as ip,user_agent,action,entity_type,entity_id,tenant_id,request_id,source,data,hash,created_at FROM audit_events WHERE entity_type=$1 AND entity_id=$2 ORDER BY occurred_at DESC OFFSET $3 LIMIT $4"#)
            .bind(et)
            .bind(eid)
            .bind(offset)
            .bind(limit)
            .fetch_all(self.db)
            .await?)
    }
}

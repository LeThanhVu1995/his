use sqlx::{Pool, Postgres};
use crate::domain::entities::audit_event::AuditEvent;

pub struct AuditRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> AuditRepo<'a> {
    pub async fn insert(&self, e: &AuditEvent) -> anyhow::Result<()> {
        sqlx::query(r#"INSERT INTO audit_log(audit_id, event_time, user_id, entity_name, entity_id, action, before_json, after_json, ip_address) VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9)"#)
            .bind(&e.audit_id)
            .bind(e.event_time)
            .bind(&e.user_id)
            .bind(&e.entity_name)
            .bind(&e.entity_id)
            .bind(&e.action)
            .bind(&e.before_json)
            .bind(&e.after_json)
            .bind(&e.ip_address)
            .execute(self.db)
            .await?;
        Ok(())
    }

    pub async fn list(&self, limit: i64, offset: i64) -> anyhow::Result<Vec<AuditEvent>> {
        Ok(sqlx::query_as::<_, AuditEvent>(r#"SELECT audit_id, event_time, user_id, entity_name, entity_id, action, before_json, after_json, ip_address FROM audit_log ORDER BY event_time DESC OFFSET $1 LIMIT $2"#)
            .bind(offset)
            .bind(limit)
            .fetch_all(self.db)
            .await?)
    }

    pub async fn by_user(&self, user_id: &str, limit: i64, offset: i64) -> anyhow::Result<Vec<AuditEvent>> {
        Ok(sqlx::query_as::<_, AuditEvent>(r#"SELECT audit_id, event_time, user_id, entity_name, entity_id, action, before_json, after_json, ip_address FROM audit_log WHERE user_id = $1 ORDER BY event_time DESC OFFSET $2 LIMIT $3"#)
            .bind(user_id)
            .bind(offset)
            .bind(limit)
            .fetch_all(self.db)
            .await?)
    }

    pub async fn by_entity(&self, entity_name: &str, entity_id: &str, limit: i64, offset: i64) -> anyhow::Result<Vec<AuditEvent>> {
        Ok(sqlx::query_as::<_, AuditEvent>(r#"SELECT audit_id, event_time, user_id, entity_name, entity_id, action, before_json, after_json, ip_address FROM audit_log WHERE entity_name = $1 AND entity_id = $2 ORDER BY event_time DESC OFFSET $3 LIMIT $4"#)
            .bind(entity_name)
            .bind(entity_id)
            .bind(offset)
            .bind(limit)
            .fetch_all(self.db)
            .await?)
    }

    pub async fn by_action(&self, action: &str, limit: i64, offset: i64) -> anyhow::Result<Vec<AuditEvent>> {
        Ok(sqlx::query_as::<_, AuditEvent>(r#"SELECT audit_id, event_time, user_id, entity_name, entity_id, action, before_json, after_json, ip_address FROM audit_log WHERE action = $1 ORDER BY event_time DESC OFFSET $2 LIMIT $3"#)
            .bind(action)
            .bind(offset)
            .bind(limit)
            .fetch_all(self.db)
            .await?)
    }
}

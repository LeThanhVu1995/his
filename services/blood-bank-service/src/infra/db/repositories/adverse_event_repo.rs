use uuid::Uuid;
use sqlx::{Pool, Postgres};
use crate::domain::entities::adverse_event::BloodAdverseEvent;

pub struct AdverseEventRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> AdverseEventRepo<'a> {
    pub async fn create(&self, event: &BloodAdverseEvent) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO bb_adverse_event(event_id, issue_id, event_time, type_code, severity_code, description, created_at, updated_at)
             VALUES($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(event.event_id)
        .bind(event.issue_id)
        .bind(event.event_time)
        .bind(&event.type_code)
        .bind(&event.severity_code)
        .bind(&event.description)
        .bind(event.created_at)
        .bind(event.updated_at)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn get_by_id(&self, event_id: Uuid) -> anyhow::Result<Option<BloodAdverseEvent>> {
        Ok(sqlx::query_as::<_, BloodAdverseEvent>(
            "SELECT event_id, issue_id, event_time, type_code, severity_code, description, created_at, updated_at
             FROM bb_adverse_event WHERE event_id = $1"
        )
        .bind(event_id)
        .fetch_optional(self.db)
        .await?)
    }

    pub async fn list_by_issue(&self, issue_id: Uuid, limit: i64, offset: i64) -> anyhow::Result<Vec<BloodAdverseEvent>> {
        Ok(sqlx::query_as::<_, BloodAdverseEvent>(
            "SELECT event_id, issue_id, event_time, type_code, severity_code, description, created_at, updated_at
             FROM bb_adverse_event WHERE issue_id = $1 ORDER BY event_time DESC LIMIT $2 OFFSET $3"
        )
        .bind(issue_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.db)
        .await?)
    }

    pub async fn list_recent(&self, limit: i64) -> anyhow::Result<Vec<BloodAdverseEvent>> {
        Ok(sqlx::query_as::<_, BloodAdverseEvent>(
            "SELECT event_id, issue_id, event_time, type_code, severity_code, description, created_at, updated_at
             FROM bb_adverse_event ORDER BY event_time DESC LIMIT $1"
        )
        .bind(limit)
        .fetch_all(self.db)
        .await?)
    }
}

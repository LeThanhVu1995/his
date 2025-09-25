use uuid::Uuid;
use sqlx::{Pool, Postgres};

pub struct OutboxStore<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> OutboxStore<'a> {
    pub async fn publish(
        &self,
        topic: &str,
        key: &str,
        payload: &serde_json::Value,
    ) -> anyhow::Result<()> {
        let id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO outbox_events(id,topic,key,payload,status,created_at)
             VALUES($1,$2,$3,$4,'PENDING',NOW())"
        )
        .bind(id)
        .bind(topic)
        .bind(key)
        .bind(payload)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn mark_published(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE outbox_events SET status='PUBLISHED', published_at=NOW() WHERE id=$1"
        )
        .bind(id)
        .execute(self.db)
        .await?;
        Ok(())
    }
}

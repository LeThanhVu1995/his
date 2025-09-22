use sqlx::{Pool, Postgres};
use crate::domain::entities::webhook::Webhook;

pub struct WebhookRepo<'a> {
    pub db: &'a Pool<Postgres>,
}

impl<'a> WebhookRepo<'a> {
    pub async fn register(&self, w: &Webhook) -> anyhow::Result<()> {
        sqlx::query(
            r#"INSERT INTO notify_webhooks(id,name,url,secret,is_active) VALUES($1,$2,$3,$4,$5)"#
        )
        .bind(w.id)
        .bind(&w.name)
        .bind(&w.url)
        .bind(&w.secret)
        .bind(w.is_active)
        .execute(self.db)
        .await?;
        Ok(())
    }

    pub async fn list(&self) -> anyhow::Result<Vec<Webhook>> {
        Ok(sqlx::query_as::<_, Webhook>(
            r#"SELECT id,name,url,secret,is_active,created_at FROM notify_webhooks WHERE is_active=TRUE ORDER BY created_at DESC"#
        )
        .fetch_all(self.db)
        .await?)
    }
}

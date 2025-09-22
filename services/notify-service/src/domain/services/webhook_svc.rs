use crate::infrastructure::repositories::webhook_repo::WebhookRepo;
use crate::domain::entities::webhook::Webhook;
use reqwest::Client;

pub struct WebhookSvc<'a> {
    pub db: &'a sqlx::Pool<sqlx::Postgres>,
}

impl<'a> WebhookSvc<'a> {
    pub async fn register(&self, w: &Webhook) -> anyhow::Result<()> {
        let repo = WebhookRepo { db: self.db };
        repo.register(w).await
    }

    pub async fn list(&self) -> anyhow::Result<Vec<Webhook>> {
        let repo = WebhookRepo { db: self.db };
        repo.list().await
    }

    pub async fn trigger_all(&self, payload: &serde_json::Value) -> anyhow::Result<()> {
        let hooks = self.list().await?;
        let client = Client::new();

        for hook in hooks {
            if hook.is_active {
                let _ = client
                    .post(&hook.url)
                    .json(payload)
                    .send()
                    .await;
            }
        }

        Ok(())
    }
}

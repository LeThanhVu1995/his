use crate::config::Settings;
use crate::security::policy::{permission_catalog, PermissionDef};
use anyhow::Context;
use std::time::Duration;
use tokio::time::sleep;

#[derive(serde::Serialize)]
struct RegisterRequest {
    permissions: Vec<PermissionDef>
}

pub async fn register_permissions(cfg: &Settings) -> anyhow::Result<()> {
    let base = cfg.iam_service_base_url.as_deref().context("IAM_SERVICE_BASE_URL missing")?;
    let token = cfg.iam_service_token.as_deref().context("IAM_SERVICE_TOKEN missing")?;
    let url = format!("{}/policies/register", base);
    let body = RegisterRequest {
        permissions: permission_catalog(&cfg.service_name)
    };

    // Retry logic với exponential backoff
    let max_retries = 5;
    let mut retry_delay = Duration::from_secs(1);

    for attempt in 1..=max_retries {
        match try_register(&url, &token, &body).await {
            Ok(_) => {
                tracing::info!("Permissions registered successfully on attempt {}", attempt);
                return Ok(());
            }
            Err(e) if attempt == max_retries => {
                tracing::error!("Failed to register permissions after {} attempts: {}", max_retries, e);
                return Err(e);
            }
            Err(e) => {
                tracing::warn!("Register attempt {} failed: {}, retrying in {:?}", attempt, e, retry_delay);
                sleep(retry_delay).await;
                retry_delay = retry_delay * 2; // Exponential backoff
            }
        }
    }

    unreachable!()
}

async fn try_register(url: &str, token: &str, body: &RegisterRequest) -> anyhow::Result<()> {
    let res = reqwest::Client::new()
        .post(url)
        .bearer_auth(token)
        .json(body)
        .timeout(Duration::from_secs(10))
        .send()
        .await?;

    if !res.status().is_success() {
        let status = res.status();
        let error_text = res.text().await.unwrap_or_default();
        anyhow::bail!("HTTP {}: {}", status, error_text);
    }

    Ok(())
}

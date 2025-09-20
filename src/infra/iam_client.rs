use crate::config::Settings;
use crate::security::policy::{permission_catalog, PermissionDef};
use anyhow::Context;

#[derive(serde::Serialize)]
struct RegisterRequest {
    permissions: Vec<PermissionDef>,
}

pub async fn register_permissions(cfg: &Settings) -> anyhow::Result<()> {
    let base = cfg
        .iam_service_base_url
        .as_deref()
        .context("IAM_SERVICE_BASE_URL missing")?;
    let token = cfg
        .iam_service_token
        .as_deref()
        .context("IAM_SERVICE_TOKEN missing")?;
    let url = format!("{}/policies/register", base);
    let body = RegisterRequest {
        permissions: permission_catalog(&cfg.service_name),
    };
    let res = reqwest::Client::new()
        .post(url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;
    if !res.status().is_success() {
        anyhow::bail!("register failed: {}", res.text().await.unwrap_or_default());
    }
    Ok(())
}

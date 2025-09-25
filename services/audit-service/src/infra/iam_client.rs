use crate::config::Settings;
use crate::security::policy::PermissionDef;

pub async fn register_permissions(cfg: &Settings) -> anyhow::Result<()> {
    if let (Some(base_url), Some(token)) = (&cfg.iam_service_base_url, &cfg.iam_service_token) {
        let client = reqwest::Client::new();
        let permissions: Vec<PermissionDef> = crate::security::policy::permission_catalog(&cfg.service_name);
        let res = client
            .post(format!("{}/permissions:register", base_url))
            .bearer_auth(token)
            .json(&permissions)
            .send()
            .await?
            .error_for_status()?;
        tracing::info!("IAM permissions registered: {:?}", res.status());
    } else {
        tracing::warn!("IAM service base URL or token not configured. Skipping permission registration.");
    }
    Ok(())
}

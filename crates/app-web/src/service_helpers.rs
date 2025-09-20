// src/service_helpers.rs - Simple service helper functions
use std::time::Duration;
use crate::security::PermissionDef;

/// Register permissions with IAM service (with retry logic)
pub async fn register_permissions_with_retry<F>(
    service_name: &str,
    iam_service_base_url: Option<&str>,
    iam_service_token: Option<&str>,
    permission_catalog: F,
    set_permissions_registered: fn(bool),
) where
    F: Fn(&str) -> Vec<PermissionDef>,
{
    if let (Some(base_url), Some(token)) = (iam_service_base_url, iam_service_token) {
        let permissions = permission_catalog(service_name);

        match try_register_permissions(base_url, token, &permissions).await {
            Ok(_) => {
                tracing::info!("Permissions registered successfully");
                set_permissions_registered(true);
            }
            Err(e) => {
                tracing::warn!(?e, "register perm failed, will retry in background");
                set_permissions_registered(false);

                // Spawn background retry task
                let base_url = base_url.to_string();
                let token = token.to_string();
                let permissions = permissions.clone();
                actix_rt::spawn(async move {
                    let mut retry_interval = tokio::time::interval(Duration::from_secs(30));
                    loop {
                        retry_interval.tick().await;
                        match try_register_permissions(&base_url, &token, &permissions).await {
                            Ok(_) => {
                                tracing::info!("Background permission registration successful");
                                set_permissions_registered(true);
                                break;
                            }
                            Err(e) => {
                                tracing::warn!(?e, "Background permission registration failed, will retry");
                            }
                        }
                    }
                });
            }
        }
    }
}

async fn try_register_permissions(
    base_url: &str,
    token: &str,
    permissions: &[PermissionDef],
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/policies/register", base_url);
    let body = serde_json::json!({
        "permissions": permissions
    });

    // Retry logic với exponential backoff
    let max_retries = 5;
    let mut retry_delay = Duration::from_secs(1);

    for attempt in 1..=max_retries {
        match try_register_once(&url, token, &body).await {
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
                tokio::time::sleep(retry_delay).await;
                retry_delay = retry_delay * 2; // Exponential backoff
            }
        }
    }

    unreachable!()
}

async fn try_register_once(url: &str, token: &str, body: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
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
        return Err(format!("HTTP {}: {}", status, error_text).into());
    }

    Ok(())
}

/// Macro to create a simple service main function
#[macro_export]
macro_rules! service_main {
    (
        service_name: $service_name:expr,
        config: $config:expr,
        permission_catalog: $permission_catalog:expr,
        set_permissions_registered: $set_permissions_registered:expr,
        configure_app: $configure_app:expr,
        validator: $validator:expr
    ) => {{
        use app_web::service_helpers::register_permissions_with_retry;
        use actix_cors::Cors;
        use actix_governor::{Governor, GovernorConfigBuilder};
        use app_web::prelude::{AuthMiddleware, AuthConfig};
        use actix_web::{App, HttpServer, middleware::Logger, web};

        // Register permissions with retry
        register_permissions_with_retry(
            &$config.service_name,
            $config.iam_service_base_url.as_deref(),
            $config.iam_service_token.as_deref(),
            $permission_catalog,
            $set_permissions_registered,
        ).await;

        // Start server
        let pool = sqlx::PgPool::connect(&$config.database_url).await.expect("Failed to connect to database");
        let port = $config.service_port;
        let host = format!("0.0.0.0:{}", port);

        HttpServer::new(move || {
            let cors = Cors::permissive();
            let governor_conf = GovernorConfigBuilder::default().finish().unwrap();
            let auth_middleware = AuthMiddleware::new($validator, AuthConfig {
                optional: false,
                required_scopes: vec![],
                any_role: vec![],
            });

            App::new()
                .wrap(Logger::default())
                .wrap(cors)
                .wrap(Governor::new(&governor_conf))
                .wrap(auth_middleware)
                .app_data(web::Data::new(pool.clone()))
                .configure($configure_app)
        })
        .workers(2)
        .bind(host)?
        .run()
        .await
    }};
}

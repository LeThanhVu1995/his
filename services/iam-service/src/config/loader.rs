// iam-service config loader.rs placeholder
use super::types::ServiceConfig;
use std::env;

pub fn load_from_env() -> ServiceConfig {
    let service_name = env::var("SERVICE_NAME").unwrap_or_else(|_| "iam-service".into());
    let service_version: String = env::var("SERVICE_VERSION").unwrap_or_else(|_| "0.1.0".into());
    let http_host = env::var("HTTP_HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let http_port: u16 = env::var("HTTP_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8081);
    let dev_mode = env::var("DEV_MODE").map(|v| v == "true" || v == "1").unwrap_or(true);

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL missing");

    let telemetry = app_config::TelemetryConfig {
        otlp_endpoint: env::var("OTLP_ENDPOINT").unwrap_or_else(|_| "http://localhost:4317".into()),
        log_level:     env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        service_name: service_name.clone(),
        enabled: true,
    };

    let security = app_config::SecurityConfig {
        issuer: env::var("KC_ISSUER").unwrap_or_else(|_| "http://localhost:8080/realms/his".into()),
        audience: env::var("KC_AUDIENCE").unwrap_or_default(),
        jwks_ttl: Some(env::var("KC_JWKS_TTL").unwrap_or_else(|_| "10m".into())),
        realm: env::var("KC_REALM").unwrap_or_else(|_| "his".into()),
    };

    let kafka = app_config::KafkaConfig {
        brokers: env::var("KAFKA_BROKERS").unwrap_or_else(|_| "localhost:9092".into()),
        group_id: env::var("KAFKA_GROUP_ID").unwrap_or_else(|_| "iam-service".into()),
        security: env::var("KAFKA_SECURITY").ok(),
        client_id: env::var("KAFKA_CLIENT_ID").unwrap_or_else(|_| "iam-service".into()),
    };

    let oidc_login_client_id = env::var("KC_LOGIN_CLIENT_ID").unwrap_or_else(|_| "his-web".into());
    let idp_providers = env::var("KC_LOGIN_PROVIDERS")
        .unwrap_or_else(|_| "azure,google,cognito".into())
        .split(',')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();

    ServiceConfig { service_name: service_name.clone(), service_version, http_host, http_port, dev_mode, database_url, telemetry, security, kafka, oidc_login_client_id, idp_providers }
}
